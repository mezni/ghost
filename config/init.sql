CREATE TABLE countries (
    id SERIAL PRIMARY KEY,
    country_name TEXT NOT NULL,
    iso TEXT NOT NULL
);

CREATE TABLE operators (
    id SERIAL PRIMARY KEY,
    operator_name TEXT NOT NULL,
    country_id INT REFERENCES countries(id)
);

CREATE TABLE plans (
    id SERIAL PRIMARY KEY,
    country_id INT REFERENCES countries(id),
    operator_id INT REFERENCES operators(id),
    percentage DOUBLE PRECISION NOT NULL CHECK (percentage >= 0 AND percentage <= 100)
);
