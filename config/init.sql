-------------------------------------------
-- DIMENSIONS
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

CREATE TABLE IF NOT EXISTS dim_operators (
    operator_id SERIAL PRIMARY KEY,
    operator_name VARCHAR(100) NOT NULL,
    brand_name VARCHAR(100),
    country_id INTEGER REFERENCES dim_countries(country_id), 
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100) NOT NULL,
    updated_at TIMESTAMP NULL,
    updated_by VARCHAR(100) NULL
);

-------------------------------------------
-- CONFIGURATIONS 
-------------------------------------------


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

CREATE TABLE ldr_operators (
    tadig TEXT,
    plmn TEXT,
    mcc TEXT,
    mnc TEXT,
    t2g TEXT,
    t3g TEXT,
    lte TEXT,
    operator TEXT,
    brand TEXT,
    country_iso TEXT,
    created_by TEXT DEFAULT 'system',
    created_at TIMESTAMP DEFAULT NOW()
);

COPY ldr_countries (iso,common_name,name_en,name_fr,prefix,prefix_flag)
FROM '/countries.csv'
DELIMITER ',' CSV HEADER;

COPY ldr_operators (tadig,plmn,mcc,mnc,t2g,t3g,lte,operator,brand,country_iso)
FROM '/operators.csv'
DELIMITER ',' CSV HEADER;

INSERT INTO dim_countries(iso_code, country_name, created_by) 
SELECT iso, common_name, created_by FROM ldr_countries
ORDER BY iso;

INSERT INTO dim_operators(operator_name, brand_name, country_id, created_by) 
SELECT lope.operator, lope.brand, dcou.country_id, lope.created_by
FROM ldr_operators lope
JOIN dim_countries dcou ON dcou.iso_code = lope.country_iso 
JOIN (
    SELECT country_iso, operator, COUNT(*)
    FROM ldr_operators 
    GROUP BY country_iso, operator 
    HAVING COUNT(*) <= 1
) t ON t.country_iso = lope.country_iso
    AND t.operator = lope.operator;

DROP TABLE ldr_countries;
DROP TABLE ldr_operators;