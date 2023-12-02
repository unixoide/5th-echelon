CREATE TABLE invites (
    sender INTEGER NOT NULL REFERENCES users(id),
    receiver INTEGER NOT NULL REFERENCES users(id),
    created DATETIME DEFAULT CURRENT_TIMESTAMP
);