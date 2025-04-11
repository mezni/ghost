-- =========================
-- DIMENSION TABLES
-- =========================
CREATE TABLE IF NOT EXISTS dim_roam_type (
    id             SERIAL PRIMARY KEY,
    roam_type           TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS dim_time (
    id             SERIAL PRIMARY KEY,
    date           DATE NOT NULL,
    year           INT NOT NULL,
    quarter        INT NOT NULL,
    month          INT NOT NULL,
    day            INT NOT NULL,
    day_of_week    INT NOT NULL,
    day_name       TEXT NOT NULL,
    week_of_year   INT NOT NULL,
    is_weekend     BOOLEAN NOT NULL,
    date_text      TEXT
);

CREATE TABLE IF NOT EXISTS dim_countries (
    id          SERIAL PRIMARY KEY,
    iso         TEXT,
    name_en     TEXT,
    name_fr     TEXT,
    prefix      TEXT,
    created_at  TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_by  TEXT,
    updated_at  TIMESTAMPTZ,
    updated_by  TEXT
);

CREATE TABLE IF NOT EXISTS dim_networks (
    id          SERIAL PRIMARY KEY,
    tadig       TEXT,
    plmn        TEXT,
    mcc         TEXT,
    mnc         TEXT,
    t2g         TEXT,
    t3g         TEXT,
    lte         TEXT,
    created_at  TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_by  TEXT,
    updated_at  TIMESTAMPTZ,
    updated_by  TEXT
);

CREATE TABLE IF NOT EXISTS dim_operators (
    id          SERIAL PRIMARY KEY,
    operator    TEXT,
    brand       TEXT,
    country_id  INT,
    network_id  INT,
    created_at  TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_by  TEXT,
    updated_at  TIMESTAMPTZ,
    updated_by  TEXT
);

CREATE TABLE IF NOT EXISTS dim_prefixes (
    id          SERIAL PRIMARY KEY,
    country_id INT,
    operator_id INT,
    cc          TEXT,
    ndc         TEXT,
    prefix      TEXT,
    created_at  TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_by  TEXT,
    updated_at  TIMESTAMPTZ,
    updated_by  TEXT
);


CREATE TABLE IF NOT EXISTS dim_imsi (
    id SERIAL PRIMARY KEY,  
    roam_type_id INT,
    imsi TEXT NOT NULL    
);

CREATE INDEX idx_dim_imsi
ON dim_imsi (imsi);

CREATE TABLE IF NOT EXISTS dim_msisdn (
    id SERIAL PRIMARY KEY,
    roam_type_id INT,      
    msisdn TEXT NOT NULL    
);

CREATE INDEX idx_dim_msisdn
ON dim_msisdn (msisdn);


-- =========================
-- STAGING TABLES
-- =========================

CREATE TABLE IF NOT EXISTS stg_roam_out (
    batch_id INT NOT NULL,
    batch_date TEXT  NOT NULL,      
    imsi TEXT NOT NULL,
    msisdn TEXT NOT NULL,
    vlr_number TEXT NOT NULL,
    prefix TEXT,   
    country_id INT,
    operator_id INT 
);

CREATE TABLE IF NOT EXISTS load_operators (
    id           SERIAL PRIMARY KEY,
    tadig        TEXT,
    plmn         TEXT,
    mcc          TEXT,
    mnc          TEXT,
    t2g          TEXT,
    t3g          TEXT,
    lte          TEXT,
    operator     TEXT,
    brand        TEXT,
    country_iso  TEXT,
    created_by   TEXT DEFAULT 'system'
);

CREATE TABLE IF NOT EXISTS load_prefixes (
    id         SERIAL PRIMARY KEY,
    country    TEXT,
    operator   TEXT,
    cc         TEXT,
    ndc        TEXT,
    prefix     TEXT,
    created_by TEXT DEFAULT 'system'
);

-- =========================
-- COPY DATA
-- =========================

COPY dim_countries (iso, name_en, name_fr, prefix, created_by)
FROM '/countries.csv'
DELIMITER ',' CSV HEADER;

COPY load_operators (tadig, plmn, mcc, mnc, t2g, t3g, lte, operator, brand, country_iso)
FROM '/operators.csv'
DELIMITER ',' CSV HEADER;

COPY load_prefixes (country, operator, cc, ndc, prefix)
FROM '/prefixes.csv'
DELIMITER ',' CSV HEADER;

-- =========================
-- LOAD DIMENSIONS
-- =========================

INSERT INTO dim_roam_type (roam_type) VALUES ('IN');
INSERT INTO dim_roam_type (roam_type) VALUES ('OUT');

INSERT INTO dim_networks (tadig, plmn, mcc, mnc, t2g, t3g, lte, created_by)
SELECT tadig, plmn, mcc, mnc, t2g, t3g, lte, created_by
FROM load_operators;


INSERT INTO dim_operators (operator, brand, country_id, network_id, created_by)
SELECT ldr.operator, ldr.brand, c.id, n.id, ldr.created_by
FROM load_operators ldr
JOIN dim_countries c ON ldr.country_iso = c.iso
JOIN dim_networks n ON ldr.tadig = n.tadig;

DELETE FROM load_prefixes
WHERE prefix IN (
    SELECT prefix
    FROM (
        SELECT prefix,
               ROW_NUMBER() OVER (PARTITION BY prefix ORDER BY id) AS rn
        FROM load_prefixes
    ) t
    WHERE t.rn > 1
);

INSERT INTO dim_prefixes (country_id,cc,prefix, created_by)
SELECT id, REPLACE(prefix, '-', ''),REPLACE(prefix, '-', ''),'system'
FROM dim_countries;

INSERT INTO dim_prefixes (country_id,operator_id, cc, ndc, prefix, created_by)
SELECT opr.id, opr.id, ldr.cc, ldr.ndc, ldr.prefix, ldr.created_by
FROM load_prefixes ldr
JOIN dim_countries cnt ON cnt.name_en = ldr.country
JOIN dim_operators opr ON opr.operator = ldr.operator
WHERE opr.country_id = cnt.id;

DELETE FROM dim_prefixes WHERE prefix IS NULL;

DELETE FROM dim_prefixes
WHERE prefix IN (
    SELECT prefix
    FROM (
        SELECT prefix,
               ROW_NUMBER() OVER (PARTITION BY prefix ORDER BY id) AS rn
        FROM dim_prefixes
    ) t
    WHERE t.rn > 1
);

-- =========================
-- CLEANUP
-- =========================

DROP TABLE load_operators;
DROP TABLE load_prefixes;

-- =========================
-- LOAD DIM_TIME
-- =========================

INSERT INTO dim_time (
    date, year, quarter, month, day, day_of_week, day_name,
    week_of_year, is_weekend, date_text
)
SELECT
    d::date AS date,
    EXTRACT(YEAR FROM d) AS year,
    EXTRACT(QUARTER FROM d) AS quarter,
    EXTRACT(MONTH FROM d) AS month,
    EXTRACT(DAY FROM d) AS day,
    EXTRACT(ISODOW FROM d) AS day_of_week,
    TO_CHAR(d, 'FMDay') AS day_name,
    EXTRACT(WEEK FROM d) AS week_of_year,
    CASE WHEN EXTRACT(ISODOW FROM d) IN (6, 7) THEN TRUE ELSE FALSE END AS is_weekend,
    TO_CHAR(d, 'YYYY-MM-DD') AS date_text
FROM GENERATE_SERIES(
    (DATE_TRUNC('year', NOW()) - INTERVAL '1 year')::DATE,
    (DATE_TRUNC('year', NOW()) + INTERVAL '5 years - 1 day')::DATE,
    '1 day'::INTERVAL
) AS d;

-- =========================
-- CONFIG/EXEC TABLES
-- =========================

CREATE TABLE IF NOT EXISTS batch_execs (
    id SERIAL PRIMARY KEY,
    batch_name TEXT NOT NULL,
    source_type TEXT,
    source_name TEXT,
    start_time TIMESTAMP,
    end_time TIMESTAMP,        
    corr_id INT,
    batch_status TEXT
);


CREATE TABLE IF NOT EXISTS sor_plan (
    id SERIAL PRIMARY KEY,
    country_id INT NOT NULL,    
    operator_id INT NOT NULL,
    rate TEXT,
    routage TEXT,
    created_at  TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_by  TEXT,
    updated_at  TIMESTAMPTZ,
    updated_by  TEXT
);

-- =========================
-- FACT TABLES
-- =========================

CREATE TABLE IF NOT EXISTS fct_roam_out (
    date_id     INT NOT NULL,
    batch_id    INT NOT NULL,
    country_id  INT,
    operator_id INT,
    imsi_id     INT NOT NULL,
    msisdn_id   INT NOT NULL
);

CREATE INDEX idx_fct_roam_out_date_id
ON fct_roam_out (date_id);

CREATE INDEX idx_fct_roam_out_country_id
ON fct_roam_out (country_id);

CREATE INDEX idx_fct_roam_out_operator_id
ON fct_roam_out (operator_id);

CREATE INDEX idx_fct_roam_out_imsi_id
ON fct_roam_out (imsi_id);

CREATE INDEX idx_fct_roam_out_msisdn_id
ON fct_roam_out (msisdn_id);


CREATE TABLE IF NOT EXISTS fct_roam_in (
    date_id          INT NOT NULL,
    batch_id         INT NOT NULL,
    country_id       INT,
    operator_id      INT,
    subscriber_total INT NOT NULL,
    subscriber_active INT NOT NULL
);


CREATE INDEX idx_fct_roam_in_date_id
ON fct_roam_in (date_id);

CREATE INDEX idx_fct_roam_in_country_id
ON fct_roam_in (country_id);

CREATE INDEX idx_fct_roam_in_operator_id
ON fct_roam_in (operator_id);

