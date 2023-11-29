UPDATE users SET ubi_id = 'ABCD' WHERE id = 1001;

INSERT INTO users (id, username, password, password_hash, ubi_id) VALUES 
    (1002, 'sam_the_fisher', NULL, '$argon2id$v=19$m=16,t=2,p=1$c1hvR0JwNTMyaTQ2Nk9uMQ$JOefJqEVFQhLM580DBnqXA', 'SAM')
;

