-------------------------------------------
-- 
-------------------------------------------

CREATE TABLE IF NOT EXISTS dim_countries (
    country_id SERIAL PRIMARY KEY,
    iso_code VARCHAR(100) NOT NULL UNIQUE,
    country_name VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100) NOT NULL,
    updated_at TIMESTAMP NULL,
    updated_by VARCHAR(100) NULL
);

-------------------------------------------
-- INITIAL LOAD 
-------------------------------------------
CREATE TABLE ldr_countries (
    iso TEXT PRIMARY KEY,
    common_name TEXT,
    name_en TEXT,
    name_fr TEXT,
    prefix TEXT,
    prefix_flag TEXT,
    created_by TEXT DEFAULT 'system',
    created_at TIMESTAMP DEFAULT NOW()
);

COPY ldr_countries (iso,common_name,name_en,name_fr,prefix,prefix_flag)
FROM '/countries.csv'
DELIMITER ',' CSV HEADER;

INSERT INTO dim_countries(iso_code, country_name, created_by) 
SELECT iso, common_name, created_by FROM ldr_countries
ORDER BY iso;

DROP TABLE ldr_countries;