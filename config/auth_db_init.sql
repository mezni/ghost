DROP TABLE IF EXISTS users;
DROP TYPE IF EXISTS role_type;

CREATE TYPE role_type AS ENUM ('user', 'admin');

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    password TEXT NOT NULL,
    role role_type NOT NULL DEFAULT 'user',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO users (name, email, verified, password, role)
VALUES (
    'Admin',
    'admin@example.com',
    TRUE,
    '$argon2id$v=19$m=19456,t=2,p=1$LaUSL2VbiPCSOBDqNd/nNQ$xGyGMsRjfrhc+NxQ8rXzkKBt4+7KnCB9HZg6lMaU6HU',
    'admin'
);
