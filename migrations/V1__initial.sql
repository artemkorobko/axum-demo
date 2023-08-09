CREATE TABLE IF NOT EXISTS login (
    id INTEGER CONSTRAINT login_id_pk PRIMARY KEY AUTOINCREMENT,
    hash TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    created TEXT NOT NULL,
    updated TEXT NOT NULL,
    state TEXT NOT NULL DEFAULT 'ENABLED' NOT NULL,
);

CREATE UNIQUE INDEX login_email_index ON login (email);
CREATE UNIQUE INDEX login_hash_index ON login (hash);
