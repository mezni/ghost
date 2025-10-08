-------------------------------------------
-- REFERENCE 
-------------------------------------------

CREATE TABLE ref_roam_directions (
    roam_direction_id SERIAL PRIMARY KEY,
    direction VARCHAR(100) NOT NULL,
    description VARCHAR(100)
);

CREATE TABLE ref_metric_types (
    metric_type_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description VARCHAR(100) 
);

CREATE TABLE ref_routage_types (
    routage_type_id SERIAL PRIMARY KEY,
    routage_type_name VARCHAR(100) NOT NULL UNIQUE
);

CREATE TABLE ref_technology_statuses (
    technology_status_id SERIAL PRIMARY KEY,
    technology_status_name VARCHAR(100) NOT NULL UNIQUE
);

CREATE TABLE ref_metric_definitions (
    metric_definition_id SERIAL PRIMARY KEY,
    metric_type_id INTEGER NOT NULL REFERENCES ref_metric_types(metric_type_id),
    roam_direction_id INTEGER NOT NULL REFERENCES ref_roam_directions(roam_direction_id),          
    name VARCHAR(100) NOT NULL,
    description VARCHAR(100),
    is_valid BOOLEAN DEFAULT TRUE 
);

CREATE TABLE ref_rules (
    rule_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description VARCHAR(100),
    is_valid BOOLEAN DEFAULT TRUE
);


CREATE TABLE IF NOT EXISTS ref_dates (
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

CREATE INDEX idx_ref_dates_date_str ON ref_dates(date_str);

INSERT INTO ref_roam_directions (direction, description) 
VALUES 
    ('IN', 'ROAM IN'),
    ('OUT', 'ROAM OUT');

INSERT INTO ref_metric_types (name, description) 
VALUES 
    ('GLOBAL', 'GLOBAL'),
    ('COUNTRY', 'COUNTRY'),
    ('OPERATOR', 'OPERATOR'),
    ('SUBSCRIBER', 'SUBSCRIBER');

INSERT INTO ref_routage_types (routage_type_name) 
VALUES
    ('Bilateral'),
    ('Orange Hub'),
    ('Comfone'),
    ('N/A');

INSERT INTO ref_technology_statuses (technology_status_name) 
VALUES
    ('Yes'),
    ('No'),
    ('Stopped'),
    ('Planned'),
    ('N/A');

INSERT INTO ref_rules (name, description)
VALUES 
    ('imsi_is_not_local', 'IMSI non local'),
    ('local_vlr_number', 'vlr_number Local'),
    ('sor_plan_bar', 'Barring operator'),
    ('sor_plan_deviation', 'Deviation SoR');

INSERT INTO ref_metric_definitions (metric_type_id, roam_direction_id, name, description)
VALUES 
    (
        (SELECT metric_type_id FROM ref_metric_types WHERE name = 'GLOBAL'),    
        (SELECT roam_direction_id FROM ref_roam_directions WHERE direction = 'IN'),
        'number_subscribers_in',
        'Number of subscribers IN'
    ),
    (
        (SELECT metric_type_id FROM ref_metric_types WHERE name = 'COUNTRY'),    
        (SELECT roam_direction_id FROM ref_roam_directions WHERE direction = 'IN'),
        'number_subscribers_in_by_country',
        'Number of subscribers IN by country'
    ),
    (
        (SELECT metric_type_id FROM ref_metric_types WHERE name = 'OPERATOR'),    
        (SELECT roam_direction_id FROM ref_roam_directions WHERE direction = 'IN'),
        'number_subscribers_in_by_operator',
        'Number of subscribers IN by operator'
    ),
    (
        (SELECT metric_type_id FROM ref_metric_types WHERE name = 'SUBSCRIBER'),    
        (SELECT roam_direction_id FROM ref_roam_directions WHERE direction = 'IN'),
        'number_subscribers_in_by_subscriber',
        'Number of subscribers IN by subscriber'
    ),
    (
        (SELECT metric_type_id FROM ref_metric_types WHERE name = 'GLOBAL'),    
        (SELECT roam_direction_id FROM ref_roam_directions WHERE direction = 'OUT'),
        'number_subscribers_out',
        'Number of subscribers OUT'
    ),
    (
        (SELECT metric_type_id FROM ref_metric_types WHERE name = 'COUNTRY'),    
        (SELECT roam_direction_id FROM ref_roam_directions WHERE direction = 'OUT'),
        'number_subscribers_out_by_country',
        'Number of subscribers OUT by country'
    ),
    (
        (SELECT metric_type_id FROM ref_metric_types WHERE name = 'OPERATOR'),    
        (SELECT roam_direction_id FROM ref_roam_directions WHERE direction = 'OUT'),
        'number_subscribers_out_by_operator',
        'Number of subscribers OUT by operator'
    ),
    (
        (SELECT metric_type_id FROM ref_metric_types WHERE name = 'SUBSCRIBER'),    
        (SELECT roam_direction_id FROM ref_roam_directions WHERE direction = 'OUT'),
        'number_subscribers_out_by_subscriber',
        'Number of subscribers OUT by subscriber'
    );

INSERT INTO ref_dates (
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
-- CONFIGURATION 
-------------------------------------------
CREATE TABLE IF NOT EXISTS cfg_countries (
    country_id SERIAL PRIMARY KEY,
    iso_code VARCHAR(100) NOT NULL,
    country_name VARCHAR(100) NOT NULL,
    is_valid BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100) NOT NULL,
    updated_at TIMESTAMP NULL,
    updated_by VARCHAR(100) NULL
);

CREATE TABLE IF NOT EXISTS cfg_operators (
    operator_id SERIAL PRIMARY KEY,
    operator_name VARCHAR(100) NOT NULL,
    brand_name VARCHAR(100),
    country_id INTEGER REFERENCES cfg_countries(country_id),
    is_valid BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100) NOT NULL,
    updated_at TIMESTAMP NULL,
    updated_by VARCHAR(100) NULL
);

CREATE TABLE IF NOT EXISTS cfg_networks (
    network_id SERIAL PRIMARY KEY,
    plmn_code VARCHAR(100) NOT NULL,
    plmn VARCHAR(100) NOT NULL,
    mcc VARCHAR(100) NOT NULL,
    mnc VARCHAR(100) NOT NULL,
    operator_id INTEGER REFERENCES cfg_operators(operator_id),
    tech_2g BOOLEAN DEFAULT FALSE,
    tech_3g BOOLEAN DEFAULT FALSE,
    tech_lte BOOLEAN DEFAULT FALSE,
    is_valid BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100) NOT NULL,
    updated_at TIMESTAMP NULL,
    updated_by VARCHAR(100) NULL
);

CREATE TABLE IF NOT EXISTS cfg_prefixes (
    prefix_id SERIAL PRIMARY KEY,
    country_id INTEGER REFERENCES cfg_countries(country_id), 
    operator_id INTEGER REFERENCES cfg_operators(operator_id), 
    prefix VARCHAR(100),
    is_valid BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100) NOT NULL,
    updated_at TIMESTAMP NULL,
    updated_by VARCHAR(100) NULL
);

CREATE TABLE cfg_subscribers (
    subscriber_id SERIAL PRIMARY KEY,
    imsi VARCHAR(100) NOT NULL,
    msisdn VARCHAR(100) NOT NULL,
    roam_direction_id INTEGER NOT NULL REFERENCES ref_roam_directions(roam_direction_id),
    first_seen TIMESTAMP,
    last_seen TIMESTAMP    
);


CREATE TABLE IF NOT EXISTS cfg_sor_plan (
    sor_plan_id SERIAL PRIMARY KEY,
    operator_id INTEGER REFERENCES cfg_operators(operator_id), 
    routage_type_id INTEGER REFERENCES ref_routage_types(routage_type_id),
    barring BOOLEAN DEFAULT FALSE,    
    rate VARCHAR(100),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100) NOT NULL,
    updated_at TIMESTAMP NULL,
    updated_by VARCHAR(100) NULL,
    is_current BOOLEAN DEFAULT TRUE
);

-------------------------------------------
-- TRANSACTIONS 
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

CREATE TABLE IF NOT EXISTS trx_metrics_global (
    metric_id SERIAL PRIMARY KEY,
    metric_definition_id INTEGER NOT NULL  REFERENCES ref_metric_definitions(metric_definition_id), 
    batch_id INTEGER NOT NULL REFERENCES batch_execs(batch_id), 
    date_id INTEGER NOT NULL REFERENCES ref_dates(date_id),
    value BIGINT
);

CREATE INDEX idx_trx_metrics_global_date_id ON trx_metrics_global (date_id);

CREATE TABLE IF NOT EXISTS trx_metrics_country (
    metric_id SERIAL PRIMARY KEY,
    metric_definition_id INTEGER NOT NULL  REFERENCES ref_metric_definitions(metric_definition_id), 
    batch_id INTEGER NOT NULL REFERENCES batch_execs(batch_id), 
    date_id INTEGER NOT NULL REFERENCES ref_dates(date_id),
    country_id INTEGER REFERENCES cfg_countries(country_id),
    value BIGINT
);

CREATE INDEX idx_trx_metrics_country_date_id ON trx_metrics_country (date_id);


CREATE TABLE IF NOT EXISTS trx_metrics_operator (
    metric_id SERIAL PRIMARY KEY,
    metric_definition_id INTEGER NOT NULL  REFERENCES ref_metric_definitions(metric_definition_id), 
    batch_id INTEGER NOT NULL REFERENCES batch_execs(batch_id), 
    date_id INTEGER NOT NULL REFERENCES ref_dates(date_id),
    country_id INTEGER REFERENCES cfg_countries(country_id),
    operator_id INTEGER REFERENCES cfg_operators(operator_id),
    value BIGINT
);

CREATE INDEX idx_trx_metrics_operator_date_id ON trx_metrics_operator (date_id);

CREATE TABLE IF NOT EXISTS trx_metrics_subscriber (
    metric_id SERIAL PRIMARY KEY,
    metric_definition_id INTEGER NOT NULL  REFERENCES ref_metric_definitions(metric_definition_id), 
    batch_id INTEGER NOT NULL REFERENCES batch_execs(batch_id), 
    date_id INTEGER NOT NULL REFERENCES ref_dates(date_id),
    country_id INTEGER REFERENCES cfg_countries(country_id),
    operator_id INTEGER REFERENCES cfg_operators(operator_id),
    subscriber_id INTEGER REFERENCES cfg_subscribers(subscriber_id), 
    value BIGINT
);

CREATE INDEX idx_trx_metrics_subscriber_date_id ON trx_metrics_subscriber (date_id);


-- country operator country_count operator_count operator_percent
-- Tunisia Orange 2000 1000  50 

CREATE TABLE IF NOT EXISTS trx_perf_out (
    perf_id SERIAL PRIMARY KEY,
    batch_id INTEGER NOT NULL REFERENCES batch_execs(batch_id), 
    date_id INTEGER NOT NULL REFERENCES ref_dates(date_id),
    country_id INTEGER REFERENCES cfg_countries(country_id),
    operator_id INTEGER REFERENCES cfg_operators(operator_id),
    country_count BIGINT,
    operator_count BIGINT,
    target_percentage BIGINT,
    actual_percentage BIGINT,
    is_outside_tolerance BOOLEAN    
);

CREATE TABLE IF NOT EXISTS trx_notifications (
    notification_id SERIAL PRIMARY KEY,
    batch_id INTEGER NOT NULL REFERENCES batch_execs(batch_id), 
    date_id INTEGER NOT NULL REFERENCES ref_dates(date_id),
    rule_id INTEGER NOT NULL REFERENCES ref_rules(rule_id),
    ref_id INT,
    message VARCHAR(255) NULL
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

INSERT INTO cfg_countries (iso_code, country_name, created_by) 
SELECT iso, common_name, created_by
FROM ldr_countries
ORDER BY iso;

INSERT INTO cfg_operators (operator_name, brand_name, country_id, created_by) 
SELECT lope.operator, lope.brand, dcou.country_id, lope.created_by
FROM ldr_operators lope
JOIN cfg_countries dcou 
    ON dcou.iso_code = lope.country_iso
JOIN (
    SELECT country_iso, operator, COUNT(*)
    FROM ldr_operators
    GROUP BY country_iso, operator
    HAVING COUNT(*) <= 1
) t 
    ON t.country_iso = lope.country_iso
    AND t.operator = lope.operator;

INSERT INTO cfg_networks (plmn_code, plmn, mcc, mnc, operator_id, tech_2g, tech_3g, tech_lte, created_by)
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
JOIN cfg_countries dcou 
    ON dcou.iso_code = lope.country_iso
JOIN cfg_operators dope 
    ON dope.operator_name = lope.operator
    AND dope.country_id = dcou.country_id;

INSERT INTO cfg_prefixes (country_id, prefix, created_by)
SELECT 
    dcou.country_id, 
    t.prefix, 
    dcou.created_by
FROM cfg_countries dcou
JOIN (
    SELECT 
        iso, 
        REPLACE(COALESCE(prefix_item, ''), '-', '') AS prefix
    FROM ldr_countries,
         unnest(string_to_array(prefix, ',')) AS prefix_item
    WHERE prefix_flag = 'X'
) t 
ON t.iso = dcou.iso_code;

INSERT INTO cfg_prefixes (country_id, operator_id, prefix, created_by)
SELECT 
    dope.country_id, 
    dope.operator_id, 
    lpre.prefix, 
    lpre.created_by
FROM ldr_prefixes lpre
JOIN cfg_countries dcou 
    ON dcou.country_name = lpre.country
JOIN cfg_operators dope 
    ON dope.operator_name = lpre.operator 
    AND dcou.country_id = dope.country_id;

DELETE FROM cfg_prefixes
WHERE prefix IN (
    SELECT prefix
    FROM (
        SELECT 
            prefix,
            ROW_NUMBER() OVER (PARTITION BY prefix ORDER BY prefix) AS rn
        FROM cfg_prefixes
    ) t
    WHERE t.rn > 1
);

DROP TABLE ldr_countries;
DROP TABLE ldr_operators;
DROP TABLE ldr_prefixes;

-------------------------------------------
-- DEMO
-------------------------------------------

INSERT INTO batch_execs (batch_name) VALUES ('TEST');

INSERT INTO trx_metrics_operator (metric_definition_id, batch_id, date_id, country_id, operator_id, value)
SELECT 3, 1, d.date_id, o.country_id, o.operator_id, 1000 + FLOOR(RANDOM()* 1000)   
FROM ref_dates d, cfg_operators o
JOIN cfg_countries c ON o.country_id = c.country_id
WHERE date BETWEEN CURRENT_DATE - INTERVAL '45 day' AND CURRENT_DATE
AND c.iso_code in ('FR');

INSERT INTO trx_metrics_operator (metric_definition_id, batch_id, date_id, country_id, operator_id, value)
SELECT 3, 1, d.date_id, o.country_id, o.operator_id,  500 + FLOOR(RANDOM()* 500)   
FROM ref_dates d, cfg_operators o
JOIN cfg_countries c ON o.country_id = c.country_id
WHERE date BETWEEN CURRENT_DATE - INTERVAL '45 day' AND CURRENT_DATE
AND c.iso_code in ('MA', 'DZ', 'BE');

INSERT INTO trx_metrics_operator (metric_definition_id, batch_id, date_id, country_id, operator_id, value)
SELECT 3, 1, d.date_id, o.country_id, o.operator_id, 200 + FLOOR(RANDOM()* 200)   
FROM ref_dates d, cfg_operators o
JOIN cfg_countries c ON o.country_id = c.country_id
WHERE date BETWEEN CURRENT_DATE - INTERVAL '45 day' AND CURRENT_DATE
AND c.iso_code in ('EG', 'CA', 'US', 'DE', 'CI');

INSERT INTO trx_metrics_operator (metric_definition_id, batch_id, date_id, country_id, operator_id, value)
SELECT 8, 1, d.date_id, o.country_id, o.operator_id, 800 + FLOOR(RANDOM()* 800)   
FROM ref_dates d, cfg_operators o
JOIN cfg_countries c ON o.country_id = c.country_id
WHERE date BETWEEN CURRENT_DATE - INTERVAL '45 day' AND CURRENT_DATE
AND c.iso_code in ('FR');

INSERT INTO trx_metrics_operator (metric_definition_id, batch_id, date_id, country_id, operator_id, value)
SELECT 8, 1, d.date_id, o.country_id, o.operator_id,  300 + FLOOR(RANDOM()* 300)   
FROM ref_dates d, cfg_operators o
JOIN cfg_countries c ON o.country_id = c.country_id
WHERE date BETWEEN CURRENT_DATE - INTERVAL '45 day' AND CURRENT_DATE
AND c.iso_code in ('MA', 'DZ', 'BE');

INSERT INTO trx_metrics_operator (metric_definition_id, batch_id, date_id, country_id, operator_id, value)
SELECT 8, 1, d.date_id, o.country_id, o.operator_id, 100 + FLOOR(RANDOM()* 100)   
FROM ref_dates d, cfg_operators o
JOIN cfg_countries c ON o.country_id = c.country_id
WHERE date BETWEEN CURRENT_DATE - INTERVAL '45 day' AND CURRENT_DATE
AND c.iso_code in ('EG', 'CA', 'US', 'DE', 'CI');

INSERT INTO trx_metrics_country (metric_definition_id,batch_id,date_id, country_id, value)  
SELECT 2, batch_id , date_id , country_id, sum(value) 
FROM trx_metrics_operator
WHERE metric_definition_id = 3
GROUP BY batch_id , date_id , country_id;

INSERT INTO trx_metrics_country (metric_definition_id,batch_id,date_id, country_id, value)  
SELECT 7, batch_id , date_id , country_id, sum(value) 
FROM trx_metrics_operator
WHERE metric_definition_id = 8
GROUP BY batch_id , date_id , country_id;

INSERT INTO trx_metrics_global (metric_definition_id,batch_id,date_id, value)  
SELECT 1, batch_id , date_id , sum(value) 
FROM trx_metrics_operator
WHERE metric_definition_id = 3
GROUP BY batch_id , date_id;

INSERT INTO trx_metrics_global (metric_definition_id,batch_id,date_id, value)  
SELECT 6, batch_id , date_id , sum(value) 
FROM trx_metrics_operator
WHERE metric_definition_id = 8
GROUP BY batch_id , date_id ;

INSERT INTO trx_perf_out (batch_id, date_id, country_id, operator_id, country_count, operator_count, target_percentage, actual_percentage, is_outside_tolerance)
WITH percentages AS (
    SELECT 
        op.batch_id,
        op.date_id,            
        op.country_id,
        op.operator_id,
        c.value AS country_value,
        op.value AS operator_value,
        ROUND((op.value::float / c.value) * 100) as percentage
    FROM trx_metrics_operator op
    JOIN trx_metrics_country c 
        ON op.country_id = c.country_id 
        AND op.date_id = c.date_id
)
SELECT 
    batch_id,
    date_id,            
    country_id,
    operator_id,
    country_value,
    operator_value,
    5,
    percentage,
    percentage > 20 AS is_over_20
FROM percentages;  

INSERT INTO trx_notifications (batch_id, date_id, rule_id, ref_id, message)
SELECT batch_id,date_id,4, NULL, 'SoR Messages: '||nbr::text
FROM (SELECT batch_id,date_id,count(*) as nbr 
FROM trx_perf_out
GROUP BY batch_id,date_id) t;
