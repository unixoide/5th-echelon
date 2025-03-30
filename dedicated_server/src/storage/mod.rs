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
use sqlx::Executor;
use sqlx::Statement;

type Result<T> = eyre::Result<T>;

fn run<F>(future: F) -> Result<F::Output>
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

pub enum LoginError {
    NotFound,
    InvalidPassword,
}

impl Storage {
    pub fn init(logger: Logger) -> Result<Self> {
        let pool = run(async {
            let pool = SqlitePool::connect("sqlite://5th-echelon.db?mode=rwc").await?;
            // enable foreign key checks
            sqlx::query("PRAGMA foreign_keys=ON").execute(&pool).await?;
            sqlx::migrate!("src/storage/migrations").run(&pool).await?;
            Ok::<_, eyre::Error>(pool)
        })??;
        Ok(Self { logger, pool })
    }

    pub async fn login_user_async(
        &self,
        username: &str,
        password: &str,
    ) -> Result<std::result::Result<u32, LoginError>> {
        let Some((id, db_password, password_hash)) = sqlx::query_as::<_, (u32, Option<String>, Option<String>)>(
            "SELECT id, password, password_hash FROM users WHERE username = ?",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?
        else {
            warn!(self.logger, "User {} not found", username);
            return Ok(Err(LoginError::NotFound));
        };

        let maybe_id = match (db_password, password_hash) {
            (None, None) => Err(eyre!("neither password or password_hash set for user {}", id)),
            (Some(_), Some(_)) => Err(eyre!("password and password_hash set for user {}", id)),
            (Some(db_password), None) => {
                info!(self.logger, "Verify plain password of {}", username);
                if db_password == password {
                    Ok(Ok(id))
                } else {
                    Ok(Err(LoginError::InvalidPassword))
                }
            }
            (None, Some(password_hash)) => {
                info!(self.logger, "Verify password hash of {}", username);
                let parsed_hash =
                    PasswordHash::new(&password_hash).map_err(|_| eyre!("password hash parsing failed"))?;
                Ok(Argon2::default()
                    .verify_password(password.as_bytes(), &parsed_hash)
                    .map_err(|_| LoginError::InvalidPassword)
                    .and(Ok(id)))
            }
        }?;

        if let Ok(user_id) = maybe_id {
            sqlx::query("UPDATE users SET last_login = CURRENT_TIMESTAMP AND is_online=1 WHERE id = ?")
                .bind(user_id)
                .execute(&self.pool)
                .await?;
        }

        Ok(maybe_id)
    }

    pub fn login_user(&self, username: &str, password: &str) -> Result<std::result::Result<u32, LoginError>> {
        run(self.login_user_async(username, password))?
    }

    pub fn register_user(&self, username: &str, password: &str, ubi_id: Option<&str>) -> Result<()> {
        run(self.register_user_async(username, password, ubi_id))?
    }

    pub async fn register_user_async(&self, username: &str, password: &str, ubi_id: Option<&str>) -> Result<()> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), salt.as_salt())
            .map_err(|_| eyre!("password hashing failed"))?
            .to_string();
        Ok(self
            .register_user_unsafe_async(username, &password_hash, ubi_id)
            .await?)
    }

    async fn register_user_unsafe_async(
        &self,
        username: &str,
        password: &str,
        ubi_id: Option<&str>,
    ) -> sqlx::Result<()> {
        sqlx::query("INSERT INTO users (username, password_hash, ubi_id) VALUES (?, ?, ?)")
            .bind(username)
            .bind(password)
            .bind(ubi_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub fn find_password_for_user(&self, user_id: u32) -> Result<Option<String>> {
        let password = run(
            sqlx::query_as::<_, (Option<String>,)>("SELECT password FROM users WHERE id = ?")
                .bind(user_id)
                .fetch_optional(&self.pool),
        )??
        .and_then(|row| row.0);
        Ok(password)
    }

    pub fn find_user_by_ubi_id(&self, ubi_id: &str) -> Result<Option<User>> {
        run(self.find_user_by_ubi_id_async(ubi_id))?
    }

    pub async fn find_user_by_ubi_id_async(&self, ubi_id: &str) -> Result<Option<User>> {
        Ok(
            sqlx::query_as("SELECT id, username, ubi_id, is_online FROM users WHERE ubi_id = ?")
                .bind(ubi_id)
                .fetch_optional(&self.pool)
                .await?,
        )
    }

    pub fn find_user_by_id(&self, id: u32) -> Result<Option<User>> {
        run(self.find_user_by_id_async(id))?
    }

    pub async fn find_user_by_id_async(&self, id: u32) -> Result<Option<User>> {
        Ok(
            sqlx::query_as("SELECT id, username, ubi_id, is_online FROM users WHERE id = ?")
                .bind(id)
                .fetch_optional(&self.pool)
                .await?,
        )
    }

    pub fn find_user_id_by_name(&self, username: &str) -> Result<Option<u32>> {
        let uid = run(sqlx::query_as::<_, (u32,)>("SELECT id FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(&self.pool))??
        .map(|row| row.0);
        Ok(uid)
    }

    pub fn find_ubi_id_by_user_id(&self, user_id: u32) -> Result<Option<String>> {
        run(self.find_ubi_id_by_user_id_async(user_id))?
    }

    pub async fn find_ubi_id_by_user_id_async(&self, user_id: u32) -> Result<Option<String>> {
        let ubi_id = sqlx::query_as::<_, (Option<String>,)>("SELECT ubi_id FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?
            .and_then(|row| row.0);
        Ok(ubi_id)
    }

    pub fn find_username_by_user_id(&self, user_id: u32) -> Result<Option<String>> {
        run(self.find_username_by_user_id_async(user_id))?
    }

    pub async fn find_username_by_user_id_async(&self, user_id: u32) -> Result<Option<String>> {
        let ubi_id = sqlx::query_as::<_, (Option<String>,)>("SELECT username FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?
            .and_then(|row| row.0);
        Ok(ubi_id)
    }

    pub fn find_user_id_by_ubi_id(&self, ubi_id: &str) -> Result<Option<u32>> {
        run(self.find_user_id_by_ubi_id_async(ubi_id))?
    }

    pub async fn find_user_id_by_ubi_id_async(&self, ubi_id: &str) -> Result<Option<u32>> {
        let uid = sqlx::query_as::<_, (u32,)>("SELECT id FROM users WHERE ubi_id = ?")
            .bind(ubi_id)
            .fetch_optional(&self.pool)
            .await?
            .map(|row| row.0);
        Ok(uid)
    }

    pub fn create_user_session(&self, user_id: u32, key: &[u8]) -> Result<()> {
        use std::fmt::Write;
        let mut s = String::new();
        for c in key {
            write!(&mut s, "{c:02X}")?;
        }
        run(async {
            sqlx::query("INSERT INTO user_sessions (id, user_id) VALUES (?, ?)")
                .bind(s)
                .bind(user_id)
                .execute(&self.pool)
                .await?;

            sqlx::query("UPDATE users SET is_online=1 WHERE id=?")
                .bind(user_id)
                .execute(&self.pool)
                .await
        })??;

        Ok(())
    }

    pub fn delete_user_session(&self, user_id: u32) -> Result<()> {
        run(async {
            sqlx::query("DELETE FROM station_urls WHERE user_id = ?")
                .bind(user_id)
                .execute(&self.pool)
                .await?;
            sqlx::query("UPDATE game_sessions SET destroyed_at=CURRENT_TIMESTAMP WHERE creator_id = ?")
                .bind(user_id)
                .execute(&self.pool)
                .await?;
            sqlx::query("DELETE FROM user_sessions WHERE user_id = ?")
                .bind(user_id)
                .execute(&self.pool)
                //     .await?;
                // TODO: how to keep track of online sessions?
                // sqlx::query("UPDATE users SET is_online=0 WHERE id=?")
                //     .bind(user_id)
                //     .execute(&self.pool)
                .await
        })??;

        Ok(())
    }

    pub fn invalidate_sessions(&self) -> Result<()> {
        run(async {
            sqlx::query("DELETE FROM station_urls").execute(&self.pool).await?;
            sqlx::query("DELETE FROM user_sessions").execute(&self.pool).await?;
            sqlx::query("UPDATE game_sessions SET destroyed_at=CURRENT_TIMESTAMP WHERE destroyed_at IS NULL")
                .execute(&self.pool)
                .await?;
            sqlx::query("UPDATE users SET is_online=0").execute(&self.pool).await
        })??;

        Ok(())
    }

    pub fn create_game_session(&self, user_id: u32, type_id: u32, attributes: String) -> Result<u32> {
        let id = run(
            sqlx::query("INSERT INTO game_sessions (type_id, creator_id, attributes) VALUES (?, ?, ?)")
                .bind(type_id)
                .bind(user_id)
                .bind(attributes)
                .execute(&self.pool),
        )??
        .last_insert_rowid();

        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        Ok(id as u32)
    }

    pub fn update_game_session(&self, type_id: u32, game_id: u32, attributes: String) -> Result<()> {
        let _id = run(
            sqlx::query("UPDATE game_sessions SET attributes = ? WHERE id = ? AND type_id = ?")
                .bind(attributes)
                .bind(game_id)
                .bind(type_id)
                .execute(&self.pool),
        )??;

        Ok(())
    }

    pub fn search_sessions(&self, type_id: u32, exclude_user: Option<u32>) -> Result<Vec<GameSession>> {
        let mut sessions: Vec<GameSession> = if let Some(uid) = exclude_user {
            run(sqlx::query_as(
                "SELECT type_id as session_type, id as session_id, creator_id, attributes FROM game_sessions WHERE type_id = ? AND creator_id != ? AND destroyed_at IS NULL",
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
                participant.station_urls = run(sqlx::query_as("SELECT url FROM station_urls WHERE user_id = ?")
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
    ) -> Result<()> {
        if private_participants.is_empty() && public_participants.is_empty() {
            warn!(self.logger, "Empty participant list");
            return Ok(());
        }
        let mut builder = sqlx::QueryBuilder::new("INSERT OR REPLACE INTO participants (game_id, user_id) ");

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

    pub async fn remove_participants_async(
        &self,
        _type_id: u32,
        session_id: u32,
        participants: Vec<u32>,
    ) -> Result<()> {
        let stmt = self
            .pool
            .prepare("DELETE FROM participants WHERE game_id = ? AND user_id = ?")
            .await?;

        for p in participants {
            stmt.query().bind(session_id).bind(p).execute(&self.pool).await?;
        }
        Ok(())
    }

    pub fn remove_participants(&self, type_id: u32, session_id: u32, participants: Vec<u32>) -> Result<()> {
        run(self.remove_participants_async(type_id, session_id, participants))?
    }

    pub fn delete_game_session(&self, creator_id: u32, type_id: u32, session_id: u32) -> Result<u64> {
        Ok(run(sqlx::query(
            "UPDATE game_sessions SET destroyed_at=CURRENT_TIMESTAMP WHERE creator_id = ? AND type_id = ? AND id = ?",
        )
        .bind(creator_id)
        .bind(type_id)
        .bind(session_id)
        .execute(&self.pool))??
        .rows_affected())
    }

    pub fn register_urls(&self, user_id: u32, urls: Vec<String>) -> Result<()> {
        if urls.is_empty() {
            warn!(self.logger, "Empty url list");
            return Ok(());
        }

        let mut builder = sqlx::QueryBuilder::new("INSERT OR REPLACE INTO station_urls (user_id, url) ");

        builder.push_values(urls.into_iter().map(|url| (user_id, url)), |mut b, (user_id, url)| {
            b.push_bind(user_id).push_bind(url);
        });
        let query = builder.build();
        debug!(self.logger, "SQL: {}", query.sql());
        run(query.execute(&self.pool))??;
        Ok(())
    }

    pub async fn list_users_async(&self) -> Result<Vec<User>> {
        Ok(
            sqlx::query_as("SELECT id, username, ubi_id, is_online FROM users WHERE ubi_id IS NOT NULL")
                .fetch_all(&self.pool)
                .await?,
        )
    }

    pub async fn add_invite_async(&self, sender_id: u32, receiver_id: u32) -> Result<i64> {
        info!(self.logger, "sending invite from {sender_id} to {receiver_id}");
        Ok(sqlx::query("INSERT INTO invites (sender, receiver) VALUES (?, ?)")
            .bind(sender_id)
            .bind(receiver_id)
            .execute(&self.pool)
            .await?
            .last_insert_rowid())
    }

    pub async fn take_invite_async(&self, user_id: u32) -> Result<Option<Invite>> {
        let row: Option<Invite> =
            sqlx::query_as("SELECT rowid as id, sender, receiver FROM invites WHERE receiver = ?")
                .bind(user_id)
                .fetch_optional(&self.pool)
                .await?;
        if let Some(invite) = row {
            sqlx::query("DELETE FROM invites WHERE rowid = ?")
                .bind(invite.id)
                .execute(&self.pool)
                .await?;
            Ok(Some(invite))
        } else {
            Ok(None)
        }
    }

    pub fn search_sessions_with_participants(&self, type_id: u32, participant_ids: &[u32]) -> Result<Vec<GameSession>> {
        run(self.search_sessions_with_participants_async(type_id, participant_ids))?
    }

    pub async fn search_sessions_with_participants_async(
        &self,
        type_id: u32,
        participant_ids: &[u32],
    ) -> Result<Vec<GameSession>> {
        let placeholders = std::iter::repeat('?')
            .take(participant_ids.len())
            .intersperse(',')
            .collect::<String>();
        let sql = format!(
            r"SELECT 
                    g.type_id as session_type, 
                    g.id as session_id,
                    g.creator_id,
                    g.attributes
                FROM game_sessions AS g
                WHERE type_id = ? AND destroyed_at IS NULL AND g.id IN (
                    SELECT game_id
                    FROM participants
                    WHERE user_id IN ({placeholders})
                )
            "
        );
        let mut query = sqlx::query_as(&sql).bind(type_id);

        for id in participant_ids {
            query = query.bind(id);
        }
        info!(self.logger, "Searching sessions with participants: {}", query.sql());

        let mut sessions: Vec<GameSession> = query.fetch_all(&self.pool).await?;

        for session in &mut sessions {
            session.participants = sqlx::query_as(
                r"
                SELECT
                    user_id,
                    username as name
                FROM participants p, users u
                WHERE u.id = user_id AND game_id = ?
                ",
            )
            .bind(session.session_id)
            .fetch_all(&self.pool)
            .await?;

            for participant in &mut session.participants {
                participant.station_urls = sqlx::query_as(
                    r"
                    SELECT url
                    FROM station_urls
                    WHERE user_id = ?
                    ",
                )
                .bind(participant.user_id)
                .fetch_all(&self.pool)
                .await?
                .into_iter()
                .map(|r: (String,)| r.0)
                .collect();
            }
        }
        Ok(sessions)
    }

    pub async fn delete_user_async(&self, user_id: u32) -> Result<()> {
        sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn list_urls(&self, user_id: u32) -> Result<Vec<String>> {
        Ok(sqlx::query_as("SELECT url FROM station_urls WHERE user_id = ?")
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|r: (String,)| r.0)
            .collect())
    }

    pub async fn list_game_sessions_async(&self) -> Result<Vec<GameSession>> {
        let mut sessions: Vec<GameSession> = sqlx::query_as(
            r"
        SELECT 
            g.type_id as session_type, 
            g.id as session_id,
            g.creator_id,
            g.attributes
        FROM game_sessions AS g
        WHERE destroyed_at IS NULL
        ",
        )
        .fetch_all(&self.pool)
        .await?;

        for session in &mut sessions {
            session.participants = sqlx::query_as(
                r"
                SELECT
                    user_id,
                    username as name
                FROM participants p, users u
                WHERE u.id = user_id AND game_id = ?
                ",
            )
            .bind(session.session_id)
            .fetch_all(&self.pool)
            .await?;

            for participant in &mut session.participants {
                participant.station_urls = sqlx::query_as(
                    r"
                    SELECT url
                    FROM station_urls
                    WHERE user_id = ?
                    ",
                )
                .bind(participant.user_id)
                .fetch_all(&self.pool)
                .await?
                .into_iter()
                .map(|r: (String,)| r.0)
                .collect();
            }
        }
        Ok(sessions)
    }

    pub async fn delete_game_session_by_id_async(&self, session_id: u32) -> Result<()> {
        sqlx::query("UPDATE game_sessions SET destroyed_at=CURRENT_TIMESTAMP WHERE id = ?")
            .bind(session_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub ubi_id: String,
    pub is_online: bool,
}

#[derive(Debug, sqlx::FromRow)]
pub struct GameSession {
    pub session_type: u32,
    pub session_id: u32,
    pub creator_id: u32,
    pub attributes: String,
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

#[derive(Debug, sqlx::FromRow)]
pub struct Invite {
    pub id: i64,
    pub sender: u32,
    pub receiver: u32,
}
