use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::Argon2;
use argon2::PasswordHash;
use argon2::PasswordHasher;
use argon2::PasswordVerifier;
use eyre::eyre;
use slog::Logger;
use sqlx::sqlite::SqlitePool;
use sqlx::Execute;

fn run<F>(future: F) -> eyre::Result<F::Output>
where
    F: std::future::Future,
{
    Ok(tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()?
        .block_on(future))
}

pub struct Storage {
    logger: Logger,
    pool: SqlitePool,
}

impl Storage {
    pub fn init(logger: Logger) -> eyre::Result<Self> {
        let pool = run(async {
            let pool = SqlitePool::connect("sqlite://5th-echelon.db?mode=rwc").await?;
            // enable foreign key checks
            sqlx::query("PRAGMA foreign_keys=ON").execute(&pool).await?;
            sqlx::migrate!("src/storage/migrations").run(&pool).await?;
            Ok::<_, eyre::Error>(pool)
        })??;
        Ok(Self { logger, pool })
    }

    pub fn login_user(&self, username: &str, password: &str) -> eyre::Result<Option<u32>> {
        let Some((id, db_password, password_hash)) =
            run(sqlx::query_as::<_, (u32, Option<String>, Option<String>)>(
                "SELECT id, password, password_hash FROM users WHERE username = ?",
            )
            .bind(username)
            .fetch_optional(&self.pool))??
        else {
            warn!(self.logger, "User {} not found", username);
            return Ok(None);
        };

        let maybe_id = match (db_password, password_hash) {
            (None, None) => Err(eyre!(
                "neither password or password_hash set for user {}",
                id
            )),
            (Some(_), Some(_)) => Err(eyre!("password and password_hash set for user {}", id)),
            (Some(db_password), None) => {
                info!(self.logger, "Verify plain password of {}", username);
                if db_password == password {
                    Ok(Some(id))
                } else {
                    Ok(None)
                }
            }
            (None, Some(password_hash)) => {
                info!(self.logger, "Verify password hash of {}", username);
                let parsed_hash = PasswordHash::new(dbg!(&password_hash))
                    .map_err(|_| eyre!("password hash parsing failed"))?;
                Ok(Argon2::default()
                    .verify_password(dbg!(password).as_bytes(), &parsed_hash)
                    .ok()
                    .and(Some(id)))
            }
        }?;

        if let Some(user_id) = maybe_id {
            run(
                sqlx::query("UPDATE users SET last_login = CURRENT_TIMESTAMP WHERE id = ?")
                    .bind(user_id)
                    .execute(&self.pool),
            )??;
        }

        Ok(maybe_id)
    }

    pub fn register_user(&self, username: &str, password: &str) -> eyre::Result<()> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), salt.as_salt())
            .map_err(|_| eyre!("password hashing failed"))?
            .to_string();
        self.register_user_unsafe(username, &password_hash)
    }

    pub fn register_user_unsafe(&self, username: &str, password: &str) -> eyre::Result<()> {
        run(
            sqlx::query("INSERT INTO users (username, password_hash) VALUES (?, ?)")
                .bind(username)
                .bind(password)
                .execute(&self.pool),
        )??;
        Ok(())
    }

    pub fn find_password_for_user(&self, user_id: u32) -> eyre::Result<Option<String>> {
        let password = run(sqlx::query_as::<_, (Option<String>,)>(
            "SELECT password FROM users WHERE id = ?",
        )
        .bind(user_id)
        .fetch_optional(&self.pool))??
        .and_then(|row| row.0);
        Ok(password)
    }

    pub fn find_user_id_by_name(&self, username: &str) -> eyre::Result<Option<u32>> {
        let uid = run(
            sqlx::query_as::<_, (u32,)>("SELECT id FROM users WHERE username = ?")
                .bind(username)
                .fetch_optional(&self.pool),
        )??
        .map(|row| row.0);
        Ok(uid)
    }

    pub fn find_ubi_id_by_user_id(&self, user_id: u32) -> eyre::Result<Option<String>> {
        let ubi_id = run(sqlx::query_as::<_, (Option<String>,)>(
            "SELECT ubi_id FROM users WHERE id = ?",
        )
        .bind(user_id)
        .fetch_optional(&self.pool))??
        .and_then(|row| row.0);
        Ok(ubi_id)
    }

    pub fn find_user_id_by_ubi_id(&self, ubi_id: &str) -> eyre::Result<Option<u32>> {
        let uid = run(
            sqlx::query_as::<_, (u32,)>("SELECT id FROM users WHERE ubi_id = ?")
                .bind(ubi_id)
                .fetch_optional(&self.pool),
        )??
        .map(|row| row.0);
        Ok(uid)
    }

    pub fn create_user_session(&self, user_id: u32, key: &[u8]) -> eyre::Result<()> {
        use std::fmt::Write;
        let mut s = String::new();
        for c in key {
            write!(&mut s, "{c:02X}")?;
        }
        run(
            sqlx::query("INSERT INTO user_sessions (id, user_id) VALUES (?, ?)")
                .bind(s)
                .bind(user_id)
                .execute(&self.pool),
        )??;

        Ok(())
    }

    pub fn delete_user_session(&self, user_id: u32) -> eyre::Result<()> {
        run(async {
            // sqlx::query("DELETE FROM user_sessions WHERE user_id = ?")
            //     .bind(user_id)
            //     .execute(&self.pool)
            //     .await?;
            sqlx::query("DELETE FROM station_urls WHERE user_id = ?")
                .bind(user_id)
                .execute(&self.pool)
                .await?;
            sqlx::query(
                "UPDATE game_sessions SET destroyed_at=CURRENT_TIMESTAMP WHERE creator_id = ?",
            )
            .bind(user_id)
            .execute(&self.pool)
            .await?;
            Ok::<_, eyre::Error>(())
        })??;

        Ok(())
    }

    pub fn create_game_session(
        &self,
        user_id: u32,
        type_id: u32,
        attributes: String,
    ) -> eyre::Result<u32> {
        let id = run(sqlx::query(
            "INSERT INTO game_sessions (type_id, creator_id, attributes) VALUES (?, ?, ?)",
        )
        .bind(type_id)
        .bind(user_id)
        .bind(attributes)
        .execute(&self.pool))??
        .last_insert_rowid();

        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        Ok(id as u32)
    }

    pub fn search_sessions(
        &self,
        type_id: u32,
        exclude_user: Option<u32>,
    ) -> eyre::Result<Vec<GameSession>> {
        let mut sessions: Vec<GameSession> = if let Some(uid) = exclude_user {
            run(sqlx::query_as(
                "SELECT type_id as session_type, id as session_id FROM game_sessions WHERE type_id = ? AND creator_id != ? AND destroyed_at IS NULL",
            )
            .bind(type_id)
            .bind(uid)
            .fetch_all(&self.pool))??
        } else {
            run(sqlx::query_as(
            "SELECT type_id as session_type, id as session_id FROM game_sessions WHERE type_id = ? AND destroyed_at IS NULL",
        )
        .bind(type_id)
        .fetch_all(&self.pool))??
        };

        for session in &mut sessions {
            session.participants = run(sqlx::query_as(
                "SELECT user_id, username as name FROM participants p, users u WHERE u.id = user_id AND game_id = ?",
            )
            .bind(session.session_id)
            .fetch_all(&self.pool))??;

            for participant in &mut session.participants {
                // Is this needed? Games seems to try to connect to itself
                if matches!(exclude_user, Some(pid) if pid == participant.user_id) {
                    continue;
                }
                participant.station_urls = run(sqlx::query_as(
                    "SELECT url FROM station_urls WHERE user_id = ?",
                )
                .bind(participant.user_id)
                .fetch_all(&self.pool))??
                .into_iter()
                .map(|r: (String,)| r.0)
                .collect();
            }
        }

        Ok(sessions)
    }

    pub fn add_participants(
        &self,
        _type_id: u32,
        session_id: u32,
        private_participants: Vec<u32>,
        public_participants: Vec<u32>,
    ) -> eyre::Result<()> {
        if private_participants.is_empty() && public_participants.is_empty() {
            warn!(self.logger, "Empty participant list");
            return Ok(());
        }
        let mut builder =
            sqlx::QueryBuilder::new("INSERT OR REPLACE INTO participants (game_id, user_id) ");

        builder.push_values(
            private_participants
                .into_iter()
                .chain(public_participants)
                .map(|user_id| (session_id, user_id)),
            |mut b, (session_id, user_id)| {
                b.push_bind(session_id).push_bind(user_id);
            },
        );
        let query = builder.build();
        debug!(self.logger, "SQL: {}", query.sql());
        run(query.execute(&self.pool))??;
        Ok(())
    }

    pub fn delete_game_session(
        &self,
        creator_id: u32,
        type_id: u32,
        session_id: u32,
    ) -> eyre::Result<u64> {
        Ok(
            run(
        sqlx::query(
                "UPDATE game_sessions SET destroyed_at=CURRENT_TIMESTAMP WHERE creator_id = ? AND type_id = ? AND id = ?")
                .bind(creator_id)
                .bind(type_id)
                .bind(session_id)
                .execute(&self.pool)
            )??
            .rows_affected()
        )
    }

    pub fn register_urls(&self, user_id: u32, urls: Vec<String>) -> eyre::Result<()> {
        if urls.is_empty() {
            warn!(self.logger, "Empty url list");
            return Ok(());
        }

        let mut builder =
            sqlx::QueryBuilder::new("INSERT OR REPLACE INTO station_urls (user_id, url) ");

        builder.push_values(
            urls.into_iter().map(|url| (user_id, url)),
            |mut b, (user_id, url)| {
                b.push_bind(user_id).push_bind(url);
            },
        );
        let query = builder.build();
        debug!(self.logger, "SQL: {}", query.sql());
        run(query.execute(&self.pool))??;
        Ok(())
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct GameSession {
    pub session_type: u32,
    pub session_id: u32,
    #[sqlx(skip)]
    pub participants: Vec<Participant>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Participant {
    pub user_id: u32,
    pub name: String,
    #[sqlx(skip)]
    pub station_urls: Vec<String>,
}
