drop table plans;
drop table operators;
drop table countries;

CREATE TABLE countries (
    country_id SERIAL PRIMARY KEY,
    country_name TEXT NOT NULL,
    iso TEXT NOT NULL,
    created_at   TIMESTAMP,
    updated_at   TIMESTAMP,
    created_by   TEXT,
    updated_by   TEXT,
    CONSTRAINT iso_unique UNIQUE (iso)
);

CREATE TABLE operators (
    operator_id SERIAL PRIMARY KEY,
    operator_name TEXT NOT NULL,
    country_id INT REFERENCES countries(country_id),
    created_at   TIMESTAMP,
    updated_at   TIMESTAMP,
    created_by   TEXT,
    updated_by   TEXT
);

CREATE TABLE plans (
    id SERIAL PRIMARY KEY,
    country_id INT REFERENCES countries(country_id),
    operator_id INT REFERENCES operators(operator_id),
    created_at   TIMESTAMP,
    updated_at   TIMESTAMP,
    created_by   TEXT,
    updated_by   TEXT,
    percentage DOUBLE PRECISION NOT NULL CHECK (percentage >= 0 AND percentage <= 100)
);
