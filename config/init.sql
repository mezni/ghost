CREATE TABLE IF NOT EXISTS dim_countries (
    country_id SERIAL PRIMARY KEY,
    iso_code VARCHAR(100) NOT NULL UNIQUE,
    country_name VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100) NOT NULL,
    updated_at TIMESTAMP NULL,
    updated_by VARCHAR(100) NULL
);