-------------------------------------------
-- BASELINE 
-------------------------------------------
CREATE TABLE cfg_roam_directions (
    roam_direction_id SERIAL PRIMARY KEY,
    direction VARCHAR(3) NOT NULL,
    description VARCHAR(100)
);

CREATE TABLE cfg_metric_types (
    metric_type_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description VARCHAR(100) 
);

CREATE TABLE cfg_metric_definitions (
    metric_definition_id SERIAL PRIMARY KEY,
    metric_type_id INTEGER NOT NULL REFERENCES cfg_metric_types(metric_type_id),
    roam_direction_id INTEGER NOT NULL REFERENCES cfg_roam_directions(roam_direction_id),          
    name VARCHAR(100) NOT NULL,
    description VARCHAR(100),
    is_valid BOOLEAN DEFAULT TRUE 
);

CREATE TABLE cfg_routage_types (
    routage_type_id SERIAL PRIMARY KEY,
    routage_type_name VARCHAR(100) NOT NULL UNIQUE
);

CREATE TABLE cfg_technology_statuses (
    technology_status_id SERIAL PRIMARY KEY,
    technology_status_name VARCHAR(100) NOT NULL UNIQUE
);

CREATE TABLE cfg_rules (
    rule_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description VARCHAR(100),
    is_active BOOLEAN
);

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

CREATE INDEX idx_dim_dates_date_str ON dim_dates(date_str);

-------------------------------------------
-- DIMENSIONS
-------------------------------------------
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
    roam_direction_id INTEGER NOT NULL REFERENCES cfg_roam_directions(roam_direction_id),
    first_seen TIMESTAMP,
    last_seen TIMESTAMP    
);

CREATE TABLE IF NOT EXISTS notifications (
    notification_id SERIAL PRIMARY KEY,
    batch_id INTEGER NOT NULL REFERENCES batch_execs(batch_id), 
    date_id INTEGER NOT NULL REFERENCES dim_dates(date_id),
    rule_id INTEGER NOT NULL REFERENCES cfg_rules(rule_id),
    ref_id INT,
    message TEXT
);

CREATE TABLE IF NOT EXISTS sor_plan (
    sor_plan_id SERIAL PRIMARY KEY,
    operator_id INTEGER REFERENCES dim_operators(operator_id), 
    routage_type_id INTEGER REFERENCES cfg_routage_types(routage_type_id),
    barring BOOLEAN DEFAULT FALSE,    
    rate VARCHAR(100),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100) NOT NULL,
    updated_at TIMESTAMP NULL,
    updated_by VARCHAR(100) NULL,
    is_current BOOLEAN DEFAULT TRUE
);

-------------------------------------------
-- METRICS 
-------------------------------------------
CREATE TABLE IF NOT EXISTS metrics_global (
    metric_id SERIAL PRIMARY KEY,
    metric_definition_id INTEGER NOT NULL  REFERENCES cfg_metric_definitions(metric_definition_id), 
    batch_id INTEGER NOT NULL REFERENCES batch_execs(batch_id), 
    date_id INTEGER NOT NULL REFERENCES dim_dates(date_id),
    value BIGINT
);

CREATE INDEX idx_metrics_global_date_id ON metrics_global (date_id);

CREATE TABLE IF NOT EXISTS metrics_country (
    metric_id SERIAL PRIMARY KEY,
    metric_definition_id INTEGER NOT NULL REFERENCES cfg_metric_definitions(metric_definition_id), 
    batch_id INTEGER NOT NULL REFERENCES batch_execs(batch_id), 
    date_id INTEGER NOT NULL REFERENCES dim_dates(date_id),
    country_id INTEGER REFERENCES dim_countries(country_id),
    value BIGINT
);
CREATE INDEX idx_metrics_country_date_id ON metrics_country (date_id);


CREATE TABLE IF NOT EXISTS metrics_operator (
    metric_id SERIAL PRIMARY KEY,
    metric_definition_id INTEGER NOT NULL REFERENCES cfg_metric_definitions(metric_definition_id), 
    batch_id INTEGER NOT NULL REFERENCES batch_execs(batch_id), 
    date_id INTEGER NOT NULL REFERENCES dim_dates(date_id),
    country_id INTEGER REFERENCES dim_countries(country_id),
    operator_id INTEGER REFERENCES dim_operators(operator_id),
    value BIGINT
);
CREATE INDEX idx_metrics_operator_date_id ON metrics_operator (date_id);


CREATE TABLE IF NOT EXISTS metrics_subscriber (
    metric_id SERIAL PRIMARY KEY,
    metric_definition_id INTEGER NOT NULL REFERENCES cfg_metric_definitions(metric_definition_id), 
    batch_id INTEGER NOT NULL REFERENCES batch_execs(batch_id), 
    date_id INTEGER NOT NULL REFERENCES dim_dates(date_id),
    country_id INTEGER REFERENCES dim_countries(country_id),
    operator_id INTEGER REFERENCES dim_operators(operator_id),
    subscriber_id INTEGER REFERENCES dim_subscribers(subscriber_id), 
    value BIGINT
);

CREATE INDEX idx_metrics_subscriber_date_id ON metrics_subscriber (date_id);

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
-- CONFIGURATIONS 
-------------------------------------------

INSERT INTO cfg_routage_types (routage_type_name) VALUES
    ('Bilateral'),
    ('Orange Hub'),
    ('Comfone'),
    ('N/A');

INSERT INTO cfg_technology_statuses (technology_status_name) VALUES
    ('Yes'),
    ('No'),
    ('Stopped'),
    ('Planned'),
    ('N/A');

INSERT INTO cfg_roam_directions (direction, description) 
VALUES 
    ('IN', 'ROAM IN'),
    ('OUT', 'ROAM OUT');

INSERT INTO cfg_rules (name , description, is_active) VALUES ('imsi_is_not_local','IMSI non local',TRUE);
INSERT INTO cfg_rules (name , description, is_active) VALUES ('local_vlr_number','vlr_number Local ',TRUE);
INSERT INTO cfg_rules (name , description, is_active) VALUES ('sor_plan_bar','Barring operator',TRUE);
INSERT INTO cfg_rules (name , description, is_active) VALUES ('sor_plan_deviation','Deviation SoR',TRUE);


INSERT INTO cfg_metric_types (name, description) 
VALUES 
    ('GLOBAL', 'GLOBAL'),
    ('COUNTRY', 'COUNTRY'),
    ('OPERATOR', 'OPERATOR'),
    ('SUBSCRIBER', 'SUBSCRIBER');

INSERT INTO cfg_metric_definitions (metric_type_id, roam_direction_id, name, description)
VALUES 
(
    (SELECT  metric_type_id FROM cfg_metric_types WHERE name = 'GLOBAL'),    
    (SELECT  roam_direction_id FROM cfg_roam_directions WHERE direction = 'IN'),
    'number_subscribers_in',
    'Number of subscribers IN'
);

INSERT INTO cfg_metric_definitions (metric_type_id, roam_direction_id, name, description)
VALUES 
(
    (SELECT  metric_type_id FROM cfg_metric_types WHERE name = 'COUNTRY'),    
    (SELECT  roam_direction_id FROM cfg_roam_directions WHERE direction = 'IN'),
    'number_subscribers_in_by_country',
    'Number of subscribers IN by country'
);


INSERT INTO cfg_metric_definitions (metric_type_id, roam_direction_id, name, description)
VALUES 
(
    (SELECT  metric_type_id FROM cfg_metric_types WHERE name = 'OPERATOR'),    
    (SELECT  roam_direction_id FROM cfg_roam_directions WHERE direction = 'IN'),
    'number_subscribers_in_by_operator',
    'Number of subscribers IN by operator'
);

INSERT INTO cfg_metric_definitions (metric_type_id, roam_direction_id, name, description)
VALUES 
(
    (SELECT  metric_type_id FROM cfg_metric_types WHERE name = 'SUBSCRIBER'),    
    (SELECT  roam_direction_id FROM cfg_roam_directions WHERE direction = 'IN'),
    'number_subscribers_in_by_subscriber',
    'Number of subscribers IN by subscriber'
);


INSERT INTO cfg_metric_definitions (metric_type_id, roam_direction_id, name, description)
VALUES 
(
    (SELECT  metric_type_id FROM cfg_metric_types WHERE name = 'GLOBAL'),    
    (SELECT  roam_direction_id FROM cfg_roam_directions WHERE direction = 'OUT'),
    'number_subscribers_out',
    'Number of subscribers OUT'
);

INSERT INTO cfg_metric_definitions (metric_type_id, roam_direction_id, name, description)
VALUES 
(
    (SELECT  metric_type_id FROM cfg_metric_types WHERE name = 'COUNTRY'),    
    (SELECT  roam_direction_id FROM cfg_roam_directions WHERE direction = 'OUT'),
    'number_subscribers_out_by_country',
    'Number of subscribers OUT by country'
);


INSERT INTO cfg_metric_definitions (metric_type_id, roam_direction_id, name, description)
VALUES 
(
    (SELECT  metric_type_id FROM cfg_metric_types WHERE name = 'OPERATOR'),    
    (SELECT  roam_direction_id FROM cfg_roam_directions WHERE direction = 'OUT'),
    'number_subscribers_out_by_operator',
    'Number of subscribers OUT by operator'
);

INSERT INTO cfg_metric_definitions (metric_type_id, roam_direction_id, name, description)
VALUES 
(
    (SELECT  metric_type_id FROM cfg_metric_types WHERE name = 'SUBSCRIBER'),    
    (SELECT  roam_direction_id FROM cfg_roam_directions WHERE direction = 'OUT'),
    'number_subscribers_out_by_subscriber',
    'Number of subscribers OUT by subscriber'
);

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





