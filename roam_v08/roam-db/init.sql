CREATE TABLE countries (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    created_at TIMESTAMPTZ,
    created_by TEXT,
    updated_at TIMESTAMPTZ,
    updated_by TEXT,
    CONSTRAINT code_unique UNIQUE (code)
);