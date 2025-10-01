-------------------------------------------
-- GLOBAL
-------------------------------------------
CREATE TABLE IF NOT EXISTS global_config (
    global_config_id SERIAL PRIMARY KEY,
    key VARCHAR(100) UNIQUE NOT NULL,
    value VARCHAR(100)
);

CREATE TABLE dim_roam_directions (
    roam_direction_id SERIAL PRIMARY KEY,
    direction VARCHAR(3) NOT NULL,
    description VARCHAR(100)
);


CREATE TABLE dim_metric_definition (
    metric_definition_id SERIAL PRIMARY KEY,
    roam_direction_id INTEGER NOT NULL REFERENCES dim_roam_directions(roam_direction_id), 
    name VARCHAR(100) NOT NULL,
    description VARCHAR(100),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100) NOT NULL,
    updated_at TIMESTAMP NULL,
    updated_by VARCHAR(100) NULL,
    is_current BOOLEAN DEFAULT TRUE 
);
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
    tech_2g     BOOLEAN DEFAULT FALSE,
    tech_3g     BOOLEAN DEFAULT FALSE,
    tech_lte    BOOLEAN DEFAULT FALSE,
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

CREATE TABLE dim_subscribers (
    subscriber_id SERIAL PRIMARY KEY,
    imsi VARCHAR(20) NOT NULL,
    msisdn VARCHAR(20) NOT NULL,
    roam_direction_id INTEGER NOT NULL REFERENCES dim_roam_directions(roam_direction_id),
    first_seen TIMESTAMP,
    last_seen TIMESTAMP    
);

CREATE TABLE IF NOT EXISTS sor_plan (
    sor_plan_id SERIAL PRIMARY KEY,
    operator_id INTEGER REFERENCES dim_operators(operator_id), 
    routage_type_id INTEGER REFERENCES dim_routage_types(routage_type_id),
    barring BOOLEAN DEFAULT FALSE,    
    rate VARCHAR(100),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100) NOT NULL,
    updated_at TIMESTAMP NULL,
    updated_by VARCHAR(100) NULL,
    is_current BOOLEAN DEFAULT TRUE,
    version INT NOT NULL
);

CREATE TABLE IF NOT EXISTS dim_dates (
    date_id SERIAL PRIMARY KEY,
    date DATE NOT NULL,
    year INT NOT NULL,
    quarter INT NOT NULL,
    month INT NOT NULL,
    day INT NOT NULL,
    day_of_week INT NOT NULL,
    day_name TEXT NOT NULL,
    week_of_year INT NOT NULL,
    is_weekend BOOLEAN NOT NULL,
    date_str VARCHAR(100)
);

-------------------------------------------
-- STAGING 
-------------------------------------------
CREATE TABLE IF NOT EXISTS stg_roam_out (
    batch_id INT NOT NULL,
    batch_date VARCHAR(100) NOT NULL,
    imsi VARCHAR(100) NOT NULL,
    msisdn VARCHAR(100) NOT NULL,
    vlr_number VARCHAR(100) NOT NULL,
    prefix VARCHAR(100),
    country_id INT,
    operator_id INT
);

CREATE TABLE IF NOT EXISTS stg_roam_in (
    batch_id INT NOT NULL,
    batch_date VARCHAR(100) NOT NULL,
    hlraddr VARCHAR(100),
    nsub VARCHAR(100),
    nsuba VARCHAR(100),
    prefix VARCHAR(100),
    country_id INT,
    operator_id INT
);

-------------------------------------------
-- METRICS 
-------------------------------------------

CREATE TABLE IF NOT EXISTS batch_execs (
    batch_id SERIAL PRIMARY KEY,
    batch_name TEXT NOT NULL,
    source_type VARCHAR(100),
    source_name VARCHAR(100),
    start_time TIMESTAMP,
    end_time TIMESTAMP,        
    corr_id INT,
    batch_status VARCHAR(100)
);


CREATE TABLE IF NOT EXISTS metrics_global (
    metric_id SERIAL PRIMARY KEY,
    metric_definition_id VARCHAR(100) NOT NULL,
    batch_id INTEGER NOT NULL REFERENCES batch_execs(batch_id), 
    date_id INTEGER NOT NULL REFERENCES dim_dates(date_id),
    value INT
);

CREATE INDEX idx_metrics_global_date_id ON metrics_global (date_id);

CREATE TABLE IF NOT EXISTS metrics_country (
    metric_id SERIAL PRIMARY KEY,
    metric_definition_id VARCHAR(100) NOT NULL,
    batch_id INTEGER NOT NULL REFERENCES batch_execs(batch_id), 
    date_id INTEGER NOT NULL REFERENCES dim_dates(date_id),
    country_id INTEGER REFERENCES dim_countries(country_id),
    value INT
);
CREATE INDEX idx_metrics_country_date_id ON metrics_country (date_id);


CREATE TABLE IF NOT EXISTS metrics_operator (
    metric_id SERIAL PRIMARY KEY,
    metric_definition_id VARCHAR(100) NOT NULL,
    batch_id INTEGER NOT NULL REFERENCES batch_execs(batch_id), 
    date_id INTEGER NOT NULL REFERENCES dim_dates(date_id),
    country_id INTEGER REFERENCES dim_countries(country_id),
    operator_id INTEGER REFERENCES dim_operators(operator_id),
    value INT
);
CREATE INDEX idx_metrics_operator_date_id ON metrics_operator (date_id);


CREATE TABLE IF NOT EXISTS metrics_subscriber (
    metric_id SERIAL PRIMARY KEY,
    metric_definition_id VARCHAR(100) NOT NULL,
    batch_id INTEGER NOT NULL REFERENCES batch_execs(batch_id), 
    date_id INTEGER NOT NULL REFERENCES dim_dates(date_id),
    country_id INTEGER REFERENCES dim_countries(country_id),
    operator_id INTEGER REFERENCES dim_operators(operator_id),
    subscriber_id INTEGER REFERENCES dim_subscribers(subscriber_id), 
    value INT
);

CREATE INDEX idx_metrics_subscriber_date_id ON metrics_subscriber (date_id);

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
INSERT INTO global_config (key,value) VALUES 
    ('HOME_COUNTRY','Tunisia'),
    ('HOME_OPERATOR','Orange');

INSERT INTO dim_roam_directions (direction, description) 
VALUES 
    ('IN', 'ROAM IN'),
    ('OUT', 'ROAM OUT');

INSERT INTO dim_metric_definition (roam_direction_id, name, description, created_by)
SELECT 
    roam_direction_id,
    'number_subscribers_in',
    'Number of subscribers IN',
    'system'
FROM dim_roam_directions
WHERE direction = 'IN';

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

INSERT INTO dim_dates (
    date, year, quarter, month, day, day_of_week, day_name,
    week_of_year, is_weekend, date_str
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
    TO_CHAR(d, 'YYYY-MM-DD') AS date_str
FROM GENERATE_SERIES(
    (DATE_TRUNC('year', NOW()) - INTERVAL '1 year')::DATE,
    (DATE_TRUNC('year', NOW()) + INTERVAL '5 years - 1 day')::DATE,
    '1 day'::INTERVAL
) AS d;

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
SELECT 
    lope.tadig,
    lope.plmn,
    lope.mcc,
    lope.mnc,
    dope.operator_id,
    CASE WHEN lope.t2g = 'Yes' THEN TRUE ELSE FALSE END,
    CASE WHEN lope.t3g = 'Yes' THEN TRUE ELSE FALSE END,
    CASE WHEN lope.lte = 'Yes' THEN TRUE ELSE FALSE END,
    lope.created_by
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
-- DROP TABLE ldr_countries;
-- DROP TABLE ldr_operators;
-- DROP TABLE ldr_prefixes;

