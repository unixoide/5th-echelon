use std::collections::HashMap;

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
    Ok(tokio::runtime::Builder::new_current_thread().enable_time().build()?.block_on(future))
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

    pub async fn login_user_async(&self, username: &str, password: &str) -> Result<std::result::Result<u32, LoginError>> {
        let Some((id, db_password, password_hash)) = sqlx::query_as::<_, (u32, Option<String>, Option<String>)>("SELECT id, password, password_hash FROM users WHERE username = ?")
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
                let parsed_hash = PasswordHash::new(&password_hash).map_err(|_| eyre!("password hash parsing failed"))?;
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
        let salt = SaltString::try_from_rng(&mut OsRng).unwrap();
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), salt.as_salt())
            .map_err(|_| eyre!("password hashing failed"))?
            .to_string();
        Ok(self.register_user_unsafe_async(username, &password_hash, ubi_id).await?)
    }

    async fn register_user_unsafe_async(&self, username: &str, password: &str, ubi_id: Option<&str>) -> sqlx::Result<()> {
        sqlx::query("INSERT INTO users (username, password_hash, ubi_id) VALUES (?, ?, ?)")
            .bind(username)
            .bind(password)
            .bind(ubi_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub fn find_password_for_user(&self, user_id: u32) -> Result<Option<String>> {
        let password = run(sqlx::query_as::<_, (Option<String>,)>("SELECT password FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_optional(&self.pool))??
        .and_then(|row| row.0);
        Ok(password)
    }

    pub fn find_user_by_ubi_id(&self, ubi_id: &str) -> Result<Option<User>> {
        run(self.find_user_by_ubi_id_async(ubi_id))?
    }

    pub async fn find_user_by_ubi_id_async(&self, ubi_id: &str) -> Result<Option<User>> {
        Ok(sqlx::query_as("SELECT id, username, ubi_id, is_online FROM users WHERE ubi_id = ?")
            .bind(ubi_id)
            .fetch_optional(&self.pool)
            .await?)
    }

    pub fn find_user_by_id(&self, id: u32) -> Result<Option<User>> {
        run(self.find_user_by_id_async(id))?
    }

    pub async fn find_user_by_id_async(&self, id: u32) -> Result<Option<User>> {
        Ok(sqlx::query_as("SELECT id, username, ubi_id, is_online FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?)
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

            sqlx::query("UPDATE users SET is_online=1 WHERE id=?").bind(user_id).execute(&self.pool).await
        })??;

        Ok(())
    }

    pub fn delete_user_session(&self, user_id: u32) -> Result<()> {
        run(async {
            sqlx::query("DELETE FROM station_urls WHERE user_id = ?").bind(user_id).execute(&self.pool).await?;
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
        let id = run(sqlx::query("INSERT INTO game_sessions (type_id, creator_id, attributes) VALUES (?, ?, ?)")
            .bind(type_id)
            .bind(user_id)
            .bind(attributes)
            .execute(&self.pool))??
        .last_insert_rowid();

        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        Ok(id as u32)
    }

    /// A session's attributes, raw as stored.
    pub fn game_session_attributes(&self, type_id: u32, session_id: u32) -> Result<Option<String>> {
        let row: Option<(String,)> = run(sqlx::query_as("SELECT attributes FROM game_sessions WHERE id = ? AND type_id = ?")
            .bind(session_id)
            .bind(type_id)
            .fetch_optional(&self.pool))??;
        Ok(row.map(|(a,)| a))
    }

    /// Splits the caller off into a fresh session carrying the same attributes.
    ///
    /// The client uses this to detach from whatever group it was in before joining somewhere
    /// else: it abandons its session, splits it, and then adds itself to whatever key came
    /// back. Answering with the key it just abandoned - as this used to - sends it straight
    /// back into the session it was trying to leave.
    ///
    /// Returns `None` when the session is unknown, so the caller can fall back to the old
    /// echo behaviour rather than inventing a session out of nothing.
    pub fn split_game_session(&self, user_id: u32, type_id: u32, session_id: u32) -> Result<Option<u32>> {
        let attributes: Option<(String,)> = run(sqlx::query_as("SELECT attributes FROM game_sessions WHERE id = ? AND type_id = ?")
            .bind(session_id)
            .bind(type_id)
            .fetch_optional(&self.pool))??;
        let Some((attributes,)) = attributes else {
            return Ok(None);
        };
        let new_id = self.create_game_session(user_id, type_id, attributes)?;
        self.add_participants(type_id, new_id, vec![], vec![user_id])?;
        info!(self.logger, "split session {session_id} of user {user_id} into new session {new_id}");
        Ok(Some(new_id))
    }

    pub fn update_game_session(&self, type_id: u32, game_id: u32, attributes: String) -> Result<()> {
        let _id = run(sqlx::query("UPDATE game_sessions SET attributes = ? WHERE id = ? AND type_id = ?")
            .bind(attributes)
            .bind(game_id)
            .bind(type_id)
            .execute(&self.pool))??;

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
            run(
                sqlx::query_as("SELECT type_id as session_type, id as session_id FROM game_sessions WHERE type_id = ? AND destroyed_at IS NULL")
                    .bind(type_id)
                    .fetch_all(&self.pool),
            )??
        };

        for session in &mut sessions {
            session.participants = run(
                sqlx::query_as("SELECT user_id, username as name FROM participants p, users u WHERE u.id = user_id AND game_id = ?")
                    .bind(session.session_id)
                    .fetch_all(&self.pool),
            )??;

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

    pub fn add_participants(&self, _type_id: u32, session_id: u32, private_participants: Vec<u32>, public_participants: Vec<u32>) -> Result<()> {
        if private_participants.is_empty() && public_participants.is_empty() {
            warn!(self.logger, "Empty participant list");
            return Ok(());
        }
        let mut builder = sqlx::QueryBuilder::new("INSERT OR REPLACE INTO participants (game_id, user_id) ");

        builder.push_values(
            private_participants.into_iter().chain(public_participants).map(|user_id| (session_id, user_id)),
            |mut b, (session_id, user_id)| {
                b.push_bind(session_id).push_bind(user_id);
            },
        );
        let query = builder.build();
        debug!(self.logger, "SQL: {}", query.sql());
        run(query.execute(&self.pool))??;
        Ok(())
    }

    pub async fn remove_participants_async(&self, _type_id: u32, session_id: u32, participants: Vec<u32>) -> Result<()> {
        let stmt = self.pool.prepare("DELETE FROM participants WHERE game_id = ? AND user_id = ?").await?;

        for p in participants {
            stmt.query().bind(session_id).bind(p).execute(&self.pool).await?;
        }
        Ok(())
    }

    pub fn remove_participants(&self, type_id: u32, session_id: u32, participants: Vec<u32>) -> Result<()> {
        run(self.remove_participants_async(type_id, session_id, participants))?
    }

    pub fn delete_game_session(&self, creator_id: u32, type_id: u32, session_id: u32) -> Result<u64> {
        Ok(run(
            sqlx::query("UPDATE game_sessions SET destroyed_at=CURRENT_TIMESTAMP WHERE creator_id = ? AND type_id = ? AND id = ?")
                .bind(creator_id)
                .bind(type_id)
                .bind(session_id)
                .execute(&self.pool),
        )??
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
        Ok(sqlx::query_as("SELECT id, username, ubi_id, is_online FROM users WHERE ubi_id IS NOT NULL")
            .fetch_all(&self.pool)
            .await?)
    }

    /// Publishes the game session a player is currently in, or clears it when `session_id`
    /// is `None`.
    ///
    /// Friends read this through [`list_users_async`]; it is what lets an accepted invitation
    /// find its destination.
    pub async fn set_advertised_session_async(&self, user_id: u32, session_id: Option<u32>, invite_only: bool, session_data: &[u8]) -> Result<()> {
        match session_id {
            Some(session_id) => {
                info!(
                    self.logger,
                    "user {user_id} advertises session {session_id} (invite_only={invite_only}, {} bytes of payload)",
                    session_data.len()
                );
                sqlx::query(
                    "INSERT INTO advertised_sessions (user_id, session_id, invite_only, session_data, updated_at)
                     VALUES (?, ?, ?, ?, CURRENT_TIMESTAMP)
                     ON CONFLICT(user_id) DO UPDATE SET
                        session_id = excluded.session_id,
                        invite_only = excluded.invite_only,
                        session_data = excluded.session_data,
                        updated_at = CURRENT_TIMESTAMP",
                )
                .bind(user_id)
                .bind(session_id)
                .bind(i32::from(invite_only))
                .bind(session_data.to_vec())
                .execute(&self.pool)
                .await?;
            }
            None => {
                info!(self.logger, "user {user_id} no longer advertises a session");
                sqlx::query("DELETE FROM advertised_sessions WHERE user_id = ?").bind(user_id).execute(&self.pool).await?;
            }
        }
        Ok(())
    }

    /// All currently advertised sessions, keyed by user: id, private flag and payload.
    pub async fn list_advertised_sessions_async(&self) -> Result<HashMap<u32, (u32, bool, Vec<u8>)>> {
        let rows: Vec<(u32, u32, i32, Option<Vec<u8>>)> = sqlx::query_as("SELECT user_id, session_id, invite_only, session_data FROM advertised_sessions")
            .fetch_all(&self.pool)
            .await?;
        Ok(rows
            .into_iter()
            .map(|(user_id, session_id, invite_only, data)| (user_id, (session_id, invite_only != 0, data.unwrap_or_default())))
            .collect())
    }

    /// Sessions the user currently hosts, newest first.
    ///
    /// Only sessions the game could actually be joined into are returned: not destroyed, the
    /// creator present as a participant, and station URLs registered. The caller picks which
    /// one an invitation should point at - the attributes decide whether it is a private room
    /// or an ordinary lobby, and that knowledge belongs in the protocol layer, not here.
    pub async fn find_host_sessions_async(&self, user_id: u32) -> Result<Vec<GameSession>> {
        Ok(sqlx::query_as(
            r"
            SELECT g.type_id AS session_type, g.id AS session_id, g.creator_id, g.attributes
            FROM game_sessions g
            INNER JOIN participants p ON p.game_id = g.id AND p.user_id = g.creator_id
            WHERE g.creator_id = ?
              AND g.destroyed_at IS NULL
            ORDER BY g.id DESC
            ",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?)
    }

    /// Records one pending invitation, optionally bound to an exact room.
    ///
    /// Only a single unconsumed invitation is kept per receiver: the event handed to the game
    /// carries a sender but no session key, so a second pending invitation could not be told
    /// apart from the first. Passing `None` for the room keeps the invitation unbound, which
    /// behaves exactly like it did before rooms were tracked.
    pub async fn add_invite_async(&self, sender_id: u32, receiver_id: u32, session_type: Option<u32>, session_id: Option<u32>) -> Result<i64> {
        match (session_type, session_id) {
            (Some(type_id), Some(id)) => info!(self.logger, "sending invite from {sender_id} to {receiver_id}, bound to session {id} (type {type_id})"),
            _ => info!(self.logger, "sending invite from {sender_id} to {receiver_id}, unbound - no active session found"),
        }

        let mut transaction = self.pool.begin().await?;
        sqlx::query("DELETE FROM invites WHERE receiver = ? OR consumed_at IS NOT NULL OR expires_at IS NULL OR expires_at <= CURRENT_TIMESTAMP")
            .bind(receiver_id)
            .execute(&mut *transaction)
            .await?;
        let id = sqlx::query("INSERT INTO invites (sender, receiver, session_type, session_id, expires_at) VALUES (?, ?, ?, ?, datetime('now', '+5 minutes'))")
            .bind(sender_id)
            .bind(receiver_id)
            .bind(session_type)
            .bind(session_id)
            .execute(&mut *transaction)
            .await?
            .last_insert_rowid();
        transaction.commit().await?;
        Ok(id)
    }

    /// Hands each invitation to the game exactly once, but keeps the room binding.
    ///
    /// The binding has to outlive the event: the client only starts looking for the session
    /// *after* it has accepted, and it may repeat that search several times. Deleting the row
    /// on delivery - as this used to do - is precisely what left those searches unanswered.
    /// The row disappears once the join succeeds, or after five minutes.
    pub async fn take_invite_async(&self, user_id: u32) -> Result<Option<Invite>> {
        let mut transaction = self.pool.begin().await?;
        sqlx::query("DELETE FROM invites WHERE consumed_at IS NOT NULL OR expires_at IS NULL OR expires_at <= CURRENT_TIMESTAMP")
            .execute(&mut *transaction)
            .await?;
        let invite: Option<Invite> = sqlx::query_as(
            r"
            SELECT rowid AS id, sender, receiver, session_type, session_id
            FROM invites
            WHERE receiver = ? AND delivered_at IS NULL AND consumed_at IS NULL AND expires_at > CURRENT_TIMESTAMP
            ORDER BY created DESC, rowid DESC
            LIMIT 1
            ",
        )
        .bind(user_id)
        .fetch_optional(&mut *transaction)
        .await?;
        if let Some(invite) = invite {
            sqlx::query("UPDATE invites SET delivered_at = CURRENT_TIMESTAMP WHERE rowid = ?")
                .bind(invite.id)
                .execute(&mut *transaction)
                .await?;
        }
        transaction.commit().await?;
        Ok(invite)
    }

    /// The room a pending invitation points at, ready to be returned as a search result.
    ///
    /// This deliberately ignores the matchmaking attributes the client sent along: a private
    /// room carries property 103 = 0 and would never match an ordinary search query. That
    /// filter is the whole reason invited players could not join.
    pub fn find_pending_invited_session(&self, receiver_id: u32, session_type: u32) -> Result<Option<GameSession>> {
        run(self.find_pending_invited_session_async(receiver_id, session_type))?
    }

    pub async fn find_pending_invited_session_async(&self, receiver_id: u32, session_type: u32) -> Result<Option<GameSession>> {
        let mut session: Option<GameSession> = sqlx::query_as(
            r"
            SELECT g.type_id AS session_type, g.id AS session_id, g.creator_id, g.attributes
            FROM invites i
            INNER JOIN game_sessions g ON g.id = i.session_id AND g.type_id = i.session_type
            INNER JOIN participants p ON p.game_id = g.id AND p.user_id = g.creator_id
            WHERE i.receiver = ?
              AND i.session_type = ?
              AND i.consumed_at IS NULL
              AND i.expires_at > CURRENT_TIMESTAMP
              AND g.creator_id = i.sender
              AND g.destroyed_at IS NULL
            ORDER BY i.created DESC, i.rowid DESC
            LIMIT 1
            ",
        )
        .bind(receiver_id)
        .bind(session_type)
        .fetch_optional(&self.pool)
        .await?;

        {
            let Some(session) = session.as_mut() else {
                return Ok(None);
            };
            session.participants = sqlx::query_as("SELECT user_id, username as name FROM participants p, users u WHERE u.id = user_id AND game_id = ?")
                .bind(session.session_id)
                .fetch_all(&self.pool)
                .await?;
            for participant in &mut session.participants {
                // Same reasoning as in search_sessions: the searching client has no use for
                // its own address and tries to connect to itself when it gets one.
                if participant.user_id == receiver_id {
                    continue;
                }
                participant.station_urls = sqlx::query_as("SELECT url FROM station_urls WHERE user_id = ?")
                    .bind(participant.user_id)
                    .fetch_all(&self.pool)
                    .await?
                    .into_iter()
                    .map(|r: (String,)| r.0)
                    .collect();
            }
        }
        Ok(session)
    }

    /// Retires the invitation once its room has actually been joined.
    ///
    /// Called after `JoinSession`, never before - an invitation consumed too early is exactly
    /// the failure this whole mechanism exists to avoid.
    pub fn consume_invite_for_session(&self, receiver_id: u32, session_type: u32, session_id: u32) -> Result<bool> {
        let affected = run(
            sqlx::query("UPDATE invites SET consumed_at = CURRENT_TIMESTAMP WHERE receiver = ? AND session_type = ? AND session_id = ? AND consumed_at IS NULL")
                .bind(receiver_id)
                .bind(session_type)
                .bind(session_id)
                .execute(&self.pool),
        )??
        .rows_affected();
        Ok(affected > 0)
    }

    /// Records an invitation into a game session for every recipient.
    ///
    /// Inviting the same player twice refreshes the pending invitation rather than adding a
    /// second one - the game lets a host click "invite" repeatedly and expects one entry.
    pub fn add_game_session_invites(&self, session_type: u32, session_id: u32, sender: u32, receivers: &[u32], message: &str) -> Result<()> {
        if receivers.is_empty() {
            warn!(self.logger, "Empty recipient list for game session invite");
            return Ok(());
        }
        let mut builder = sqlx::QueryBuilder::new("INSERT OR REPLACE INTO game_session_invites (session_type, session_id, sender, receiver, message) ");
        builder.push_values(receivers.iter().copied(), |mut b, receiver| {
            b.push_bind(session_type).push_bind(session_id).push_bind(sender).push_bind(receiver).push_bind(message);
        });
        let query = builder.build();
        debug!(self.logger, "SQL: {}", query.sql());
        run(query.execute(&self.pool))??;
        Ok(())
    }

    /// Lists pending invitations addressed to `user_id`.
    pub fn list_game_session_invites_received(&self, user_id: u32, session_type: u32, offset: u32, size: u32) -> Result<Vec<GameSessionInvite>> {
        self.list_game_session_invites("receiver", user_id, session_type, offset, size)
    }

    /// Lists pending invitations sent by `user_id`.
    pub fn list_game_session_invites_sent(&self, user_id: u32, session_type: u32, offset: u32, size: u32) -> Result<Vec<GameSessionInvite>> {
        self.list_game_session_invites("sender", user_id, session_type, offset, size)
    }

    /// Shared body of the two listings above; `column` selects the direction.
    ///
    /// `created_at` is converted to the packed date format Quazal puts on the wire right here in
    /// SQL: seconds, minutes, hours, day, month and year sit in one integer at bit offsets
    /// 0/6/12/17/22/26. Doing it in the query keeps the crate free of a date dependency, which
    /// it currently does not have at all.
    fn list_game_session_invites(&self, column: &str, user_id: u32, session_type: u32, offset: u32, size: u32) -> Result<Vec<GameSessionInvite>> {
        // `column` is never user input - it is one of the two literals above.
        let sql = format!(
            r"SELECT
                    session_type,
                    session_id,
                    sender,
                    receiver,
                    message,
                    (CAST(strftime('%Y', created_at) AS INTEGER) << 26)
                  | (CAST(strftime('%m', created_at) AS INTEGER) << 22)
                  | (CAST(strftime('%d', created_at) AS INTEGER) << 17)
                  | (CAST(strftime('%H', created_at) AS INTEGER) << 12)
                  | (CAST(strftime('%M', created_at) AS INTEGER) <<  6)
                  |  CAST(strftime('%S', created_at) AS INTEGER) AS created_at
                FROM game_session_invites
                WHERE {column} = ? AND session_type = ?
                ORDER BY id
                LIMIT ? OFFSET ?"
        );
        // A size of 0 means "no limit" on the wire; SQLite spells that -1.
        let limit = if size == 0 { -1i64 } else { i64::from(size) };
        Ok(run(sqlx::query_as(&sql)
            .bind(user_id)
            .bind(session_type)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool))??)
    }

    /// Counts pending invitations in one direction; `column` is `receiver` or `sender`.
    fn count_game_session_invites(&self, column: &str, user_id: u32, session_type: u32) -> Result<u32> {
        let sql = format!("SELECT COUNT(*) FROM game_session_invites WHERE {column} = ? AND session_type = ?");
        let (count,): (i64,) = run(sqlx::query_as(&sql).bind(user_id).bind(session_type).fetch_one(&self.pool))??;
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        Ok(count as u32)
    }

    pub fn count_game_session_invites_received(&self, user_id: u32, session_type: u32) -> Result<u32> {
        self.count_game_session_invites("receiver", user_id, session_type)
    }

    pub fn count_game_session_invites_sent(&self, user_id: u32, session_type: u32) -> Result<u32> {
        self.count_game_session_invites("sender", user_id, session_type)
    }

    /// Removes a pending invitation once it has been accepted, declined or withdrawn.
    ///
    /// The counterpart is identified by pid because that is all the client sends back: the
    /// recipient knows the sender, the sender knows the recipient.
    pub fn delete_game_session_invite(&self, session_type: u32, session_id: u32, sender: u32, receiver: u32) -> Result<u64> {
        Ok(run(sqlx::query("DELETE FROM game_session_invites WHERE session_type = ? AND session_id = ? AND sender = ? AND receiver = ?")
            .bind(session_type)
            .bind(session_id)
            .bind(sender)
            .bind(receiver)
            .execute(&self.pool))??
        .rows_affected())
    }

    pub fn search_sessions_with_participants(&self, type_id: u32, participant_ids: &[u32]) -> Result<Vec<GameSession>> {
        run(self.search_sessions_with_participants_async(type_id, participant_ids))?
    }

    pub async fn search_sessions_with_participants_async(&self, type_id: u32, participant_ids: &[u32]) -> Result<Vec<GameSession>> {
        let placeholders = std::iter::repeat('?').take(participant_ids.len()).intersperse(',').collect::<String>();
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
        sqlx::query("DELETE FROM users WHERE id = ?").bind(user_id).execute(&self.pool).await?;
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

/// A pending invitation into a game session (see the `game_session_invites` migration).
#[derive(Debug, sqlx::FromRow)]
pub struct GameSessionInvite {
    pub session_type: u32,
    pub session_id: u32,
    pub sender: u32,
    pub receiver: u32,
    pub message: String,
    /// Creation time, already packed into Quazal's on-the-wire date format.
    pub created_at: u64,
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

#[derive(Debug, Clone, Copy, sqlx::FromRow)]
pub struct Invite {
    pub id: i64,
    pub sender: u32,
    pub receiver: u32,
    /// The room this invitation points at. `None` when the host had no joinable session at
    /// the time of the invitation - the invitation is then delivered but cannot be resolved.
    pub session_type: Option<u32>,
    pub session_id: Option<u32>,
}
