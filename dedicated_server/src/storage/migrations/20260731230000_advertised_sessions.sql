-- The game session a player currently advertises to their friends.
--
-- Blacklist announces its session through UPLAY_USER_SetGameSession and, when somebody
-- accepts an invitation, looks for the inviter's session **in the friend list**. Genuine
-- Uplay carries that through Ubisoft's presence service; this table takes its place.
--
-- One row per player at most, hence user_id as the primary key: announcing a new session
-- replaces the previous one, and UPLAY_USER_ClearGameSession removes the row.
CREATE TABLE advertised_sessions (
    user_id INTEGER PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    session_id INTEGER NOT NULL,
    -- Private match: joinable through an invitation only. The game passes this along as
    -- `invite_only` and needs it back to tell a private lobby from a public one.
    invite_only INTEGER NOT NULL DEFAULT 0 CHECK (invite_only IN (0,1)),
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
