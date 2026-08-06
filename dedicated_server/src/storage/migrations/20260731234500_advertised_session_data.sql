-- The payload belonging to an advertised session.
--
-- The session id on its own is enough to get an accepted invitation past the game's lookup,
-- but not to actually enter the session: the client then opens one of its own. What it is
-- missing is the block the host passed to UPLAY_USER_SetGameSession (496 bytes: marker,
-- checksum, account id and payload). It travels through here unchanged - the server has no
-- reason to interpret it.
ALTER TABLE advertised_sessions ADD COLUMN session_data BLOB;
