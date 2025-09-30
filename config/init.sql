-------------------------------------------
-- DIMENSIONS
-------------------------------------------

CREATE TABLE dim_routage_types (
    routage_type_id   SERIAL PRIMARY KEY,
    routage_type_name VARCHAR(100) NOT NULL UNIQUE,
    created_at        TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by        VARCHAR(100) NOT NULL,
    updated_at        TIMESTAMP NULL,
    updated_by        VARCHAR(100) NULL
);

CREATE TABLE dim_technology_statuses (
    technology_status_id   SERIAL PRIMARY KEY,
    technology_status_name VARCHAR(100) NOT NULL UNIQUE,
    created_at             TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by             VARCHAR(100) NOT NULL,
    updated_at             TIMESTAMP NULL,
    updated_by             VARCHAR(100) NULL
);

CREATE TABLE IF NOT EXISTS dim_countries (
    country_id   SERIAL PRIMARY KEY,
    iso_code     VARCHAR(100) NOT NULL UNIQUE,
    country_name VARCHAR(100) NOT NULL,
    created_at   TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by   VARCHAR(100) NOT NULL,
    updated_at   TIMESTAMP NULL,
    updated_by   VARCHAR(100) NULL
);

CREATE TABLE IF NOT EXISTS dim_operators (
    operator_id   SERIAL PRIMARY KEY,
    operator_name VARCHAR(100) NOT NULL,
    brand_name    VARCHAR(100),
    country_id    INTEGER REFERENCES dim_countries(country_id), 
    created_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by    VARCHAR(100) NOT NULL,
    updated_at    TIMESTAMP NULL,
    updated_by    VARCHAR(100) NULL
);

CREATE TABLE IF NOT EXISTS dim_networks (
    network_id  SERIAL PRIMARY KEY,
    plmn_code   VARCHAR(100) NOT NULL,
    plmn        VARCHAR(100) NOT NULL,
    mcc         VARCHAR(100) NOT NULL,
    mnc         VARCHAR(100) NOT NULL,
    operator_id INTEGER REFERENCES dim_operators(operator_id), 
    tech_2g     VARCHAR(3),
    tech_3g     VARCHAR(3),
    tech_lte    VARCHAR(3),
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by  VARCHAR(100) NOT NULL,
    updated_at  TIMESTAMP NULL,
    updated_by  VARCHAR(100) NULL
);

CREATE TABLE IF NOT EXISTS dim_prefixes (
    prefix_id SERIAL PRIMARY KEY,
    country_id INTEGER REFERENCES dim_countries(country_id), 
    operator_id INTEGER REFERENCES dim_operators(operator_id), 
    prefix VARCHAR(100),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100) NOT NULL,
    updated_at TIMESTAMP NULL,
    updated_by VARCHAR(100) NULL
);

CREATE TABLE IF NOT EXISTS sor_plan (
    sor_plan_id SERIAL PRIMARY KEY,
    operator_id INTEGER REFERENCES dim_operators(operator_id), 
    routage_type_id INTEGER REFERENCES dim_routage_types(routage_type_id),
    barring VARCHAR(1),    
    rate VARCHAR(100),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100) NOT NULL,
    updated_at TIMESTAMP NULL,
    updated_by VARCHAR(100) NULL,
    is_current BOOLEAN DEFAULT TRUE,
    version INT NOT NULL
);

-------------------------------------------
-- VIEWS 
-------------------------------------------
CREATE OR REPLACE VIEW v_sor_plan AS
SELECT 
    net.plmn_code, 
    cnt.country_name, 
    opr.operator_name, 
    sor.barring, 
    sor.rate, 
    net.mcc, 
    net.mnc, 
    rtt.routage_type_name, 
    net.tech_2g, 
    net.tech_3g, 
    net.tech_lte
FROM sor_plan sor
JOIN dim_operators opr 
    ON sor.operator_id = opr.operator_id
JOIN dim_countries cnt 
    ON opr.country_id = cnt.country_id
JOIN dim_networks net 
    ON opr.operator_id = net.operator_id
JOIN dim_routage_types rtt 
    ON sor.routage_type_id = rtt.routage_type_id
WHERE sor.is_current = TRUE;

-------------------------------------------
-- CONFIGURATIONS 
-------------------------------------------

INSERT INTO dim_routage_types (routage_type_name, created_by) VALUES
    ('Bilateral', 'system'),
    ('Orange Hub', 'system'),
    ('Comfone', 'system'),
    ('N/A', 'system');

INSERT INTO dim_technology_statuses (technology_status_name, created_by) VALUES
    ('Yes', 'system'),
    ('No', 'system'),
    ('Stopped', 'system'),
    ('Planned', 'system'),
    ('N/A', 'system');

-------------------------------------------
-- INITIAL LOAD 
-------------------------------------------

CREATE TABLE ldr_countries (
    iso         TEXT PRIMARY KEY,
    common_name TEXT,
    name_en     TEXT,
    name_fr     TEXT,
    prefix      TEXT,
    prefix_flag TEXT,
    created_by  TEXT DEFAULT 'system',
    created_at  TIMESTAMP DEFAULT NOW()
);

CREATE TABLE ldr_operators (
    tadig       TEXT,
    plmn        TEXT,
    mcc         TEXT,
    mnc         TEXT,
    t2g         TEXT,
    t3g         TEXT,
    lte         TEXT,
    operator    TEXT,
    brand       TEXT,
    country_iso TEXT,
    created_by  TEXT DEFAULT 'system',
    created_at  TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS ldr_prefixes (
    country TEXT,
    operator TEXT,
    cc TEXT,
    ndc TEXT,
    prefix TEXT,
    created_by TEXT DEFAULT 'system',
    created_at TIMESTAMP DEFAULT NOW()
);

COPY ldr_countries (iso, common_name, name_en, name_fr, prefix, prefix_flag)
FROM '/countries.csv'
DELIMITER ',' CSV HEADER;

COPY ldr_operators (tadig, plmn, mcc, mnc, t2g, t3g, lte, operator, brand, country_iso)
FROM '/operators.csv'
DELIMITER ',' CSV HEADER;

COPY ldr_prefixes (country,operator,cc,ndc,prefix)
FROM '/prefixes.csv'
DELIMITER ',' CSV HEADER;

INSERT INTO dim_countries (iso_code, country_name, created_by) 
SELECT iso, common_name, created_by
FROM ldr_countries
ORDER BY iso;

INSERT INTO dim_operators (operator_name, brand_name, country_id, created_by) 
SELECT lope.operator, lope.brand, dcou.country_id, lope.created_by
FROM ldr_operators lope
JOIN dim_countries dcou 
    ON dcou.iso_code = lope.country_iso
JOIN (
    SELECT country_iso, operator, COUNT(*)
    FROM ldr_operators
    GROUP BY country_iso, operator
    HAVING COUNT(*) <= 1
) t 
    ON t.country_iso = lope.country_iso
    AND t.operator = lope.operator;

INSERT INTO dim_networks (plmn_code, plmn, mcc, mnc, operator_id, tech_2g, tech_3g, tech_lte, created_by) 
SELECT lope.tadig, lope.plmn, lope.mcc, lope.mnc, dope.operator_id, lope.t2g, lope.t3g, lope.lte, lope.created_by
FROM ldr_operators lope
JOIN dim_countries dcou 
    ON dcou.iso_code = lope.country_iso
JOIN dim_operators dope 
    ON dope.operator_name = lope.operator
    AND dope.country_id = dcou.country_id;

INSERT INTO dim_prefixes (country_id, prefix, created_by)
SELECT 
    dcou.country_id, 
    t.prefix, 
    dcou.created_by
FROM dim_countries dcou
JOIN (
    SELECT 
        iso, 
        REPLACE(COALESCE(prefix_item, ''), '-', '') AS prefix
    FROM ldr_countries,
         unnest(string_to_array(prefix, ',')) AS prefix_item
    WHERE prefix_flag = 'X'
) t 
ON t.iso = dcou.iso_code;

INSERT INTO dim_prefixes (country_id, operator_id, prefix, created_by)
SELECT 
    dope.country_id, 
    dope.operator_id, 
    lpre.prefix, 
    lpre.created_by
FROM ldr_prefixes lpre
JOIN dim_countries dcou 
    ON dcou.country_name = lpre.country
JOIN dim_operators dope 
    ON dope.operator_name = lpre.operator 
    AND dcou.country_id = dope.country_id;

DELETE FROM dim_prefixes
WHERE prefix IN (
    SELECT prefix
    FROM (
        SELECT 
            prefix,
            ROW_NUMBER() OVER (PARTITION BY prefix ORDER BY prefix) AS rn
        FROM dim_prefixes
    ) t
    WHERE t.rn > 1
);    

-- Cleanup staging tables
DROP TABLE ldr_countries;
DROP TABLE ldr_operators;
DROP TABLE ldr_prefixes;

