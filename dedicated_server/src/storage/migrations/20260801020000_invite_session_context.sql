-- Binds a friend invitation to the room it was issued for.
--
-- Why this is needed: the invitation event the Uplay layer hands to the game carries only the
-- inviter's identity, no session. The invited client then looks the session up itself through
-- GameSessionEx.SearchSessions (query_id 8), and a private room falls through the ordinary
-- attribute filter there, because its attributes do not match a matchmaking query. With this
-- binding the server can answer that particular search on purpose instead of treating it like
-- a matchmaking search.
--
-- All columns are nullable: if no active session of the host is found when the invitation is
-- issued, the invitation stays unbound and behaves as it did before.

ALTER TABLE invites ADD COLUMN session_type INTEGER;
ALTER TABLE invites ADD COLUMN session_id INTEGER;
ALTER TABLE invites ADD COLUMN delivered_at DATETIME;
ALTER TABLE invites ADD COLUMN consumed_at DATETIME;
ALTER TABLE invites ADD COLUMN expires_at DATETIME;

-- Existing rows have neither a binding nor an expiry and would be dropped by the new cleanup
-- immediately anyway. Invitations are transient, nothing of value is lost here.
DELETE FROM invites;

CREATE INDEX idx_invites_pending_receiver ON invites (receiver, session_type, consumed_at, expires_at);
