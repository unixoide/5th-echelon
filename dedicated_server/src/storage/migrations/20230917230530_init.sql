CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password TEXT,
    password_hash TEXT,
    ubi_id TEXT UNIQUE,
    last_login TEXT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- CREATE TABLE user_sessions (
--   id TEXT PRIMARY KEY,
--   user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
--   created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
-- );

CREATE TABLE game_sessions (
  id INTEGER PRIMARY KEY,
  type_id INTEGER NOT NULL,
  attributes TEXT,
  creator_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  -- session_id TEXT NOT NULL REFERENCES user_sessions(id) ON DELETE CASCADE,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  destroyed_at TEXT,
  is_private INTEGER CHECK (is_private IN (0,1))
);

CREATE TABLE participants (
  game_id INTEGER REFERENCES game_sessions(id) ON DELETE CASCADE,
  user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
  UNIQUE (game_id, user_id)
);

CREATE TABLE station_urls (
  user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  -- session_id TEXT NOT NULL REFERENCES user_sessions(id) ON DELETE CASCADE,
  url TEXT NOT NULL,
  UNIQUE (user_id, url)
);

INSERT INTO users (id, username, password, password_hash, ubi_id) VALUES 
    (1, 'Server', NULL, NULL, NULL),
    (105, 'Tracking', 'JaDe!', NULL, NULL),
    (1000, 'Foo', NULL, NULL, 'MYID'),
    (1001, 'AAAABBBB', NULL, '$argon2id$v=19$m=19456,t=2,p=1$ak/934K3+OsQ71Dogbr+Iw$Fy3aLbg2bQFXrnucys2gsBqiy2Jgv9QMBWWiPzS7VTk', 'SAM')
;

-- INSERT INTO user_sessions (id, user_id) VALUES
--   ('00112233445566778899AABBCCDDEEFF', 1000)
-- ;

-- INSERT INTO station_urls (user_id, session_id, url) VALUES
-- (1000, '00112233445566778899AABBCCDDEEFF','prudp:/address=127.0.0.1;port=19999;hdrType=00000000;RVCID=1234;type=2')
INSERT INTO station_urls (user_id, url) VALUES
  (1000, 'prudp:/address=127.0.0.1;port=19999;hdrType=00000000;RVCID=1234;type=2')
;

-- INSERT INTO game_sessions (id, type_id, creator_id, session_id) VALUES
--   (1, 1, 1000, '00112233445566778899AABBCCDDEEFF')
INSERT INTO game_sessions (id, type_id, creator_id) VALUES
  (1, 1, 1000)
;

INSERT INTO participants (game_id, user_id) VALUES
  (1, 1000)
;