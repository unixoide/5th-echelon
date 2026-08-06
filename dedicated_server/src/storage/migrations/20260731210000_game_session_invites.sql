-- Invitations into a game session, as used by private lobbies.
--
-- This is distinct from the `invites` table: that one backs the out-of-band friend invites
-- offered by the gRPC API, which merely nudge a player to launch/join. These here are the
-- in-game invitations of GameSessionProtocol (SendInvitation / AcceptInvitation / ...) and
-- therefore always refer to one concrete session.
--
-- Rows are removed once the invitation is answered (accepted or declined) or withdrawn by the
-- sender, so the table only ever holds pending invitations.
CREATE TABLE game_session_invites (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_type INTEGER NOT NULL,
    session_id INTEGER NOT NULL REFERENCES game_sessions(id) ON DELETE CASCADE,
    sender INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    receiver INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    message TEXT NOT NULL DEFAULT '',
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- Re-inviting somebody refreshes the existing invitation instead of piling up duplicates.
    UNIQUE (session_id, sender, receiver)
);

-- Both directions are queried per session type (GetInvitationsReceived / GetInvitationsSent).
CREATE INDEX idx_game_session_invites_receiver ON game_session_invites (receiver, session_type);
CREATE INDEX idx_game_session_invites_sender ON game_session_invites (sender, session_type);
