CREATE TABLE countries (
    country_id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    iso TEXT NOT NULL,
    created_at   TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at   TIMESTAMP WITH TIME ZONE,
    created_by   TEXT,
    updated_by   TEXT,
    CONSTRAINT iso_unique UNIQUE (iso)
);