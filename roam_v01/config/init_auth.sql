CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

INSERT INTO roles (name) VALUES ('admin'), ('user');

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    password TEXT NOT NULL,
    role_id INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_role FOREIGN KEY (role_id) REFERENCES roles(id)
);

CREATE FUNCTION set_default_role_id()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.role_id IS NULL THEN
        SELECT id INTO NEW.role_id FROM roles WHERE name = 'user';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_default_user_role
BEFORE INSERT ON users
FOR EACH ROW
EXECUTE FUNCTION set_default_role_id();


INSERT INTO users (name, email, verified, password, role_id)
VALUES (
    'Admin',
    'admin@example.com',
    TRUE,
    '$argon2id$v=19$m=19456,t=2,p=1$LaUSL2VbiPCSOBDqNd/nNQ$xGyGMsRjfrhc+NxQ8rXzkKBt4+7KnCB9HZg6lMaU6HU',
    (SELECT id FROM roles WHERE name = 'admin')
);
