----------------------
-- Baseline
----------------------
CREATE TABLE IF NOT EXISTS global_config (
    global_config_id SERIAL PRIMARY KEY,
    key TEXT,
    value TEXT
);

CREATE TABLE roam_directions (
    roam_direction_id SERIAL PRIMARY KEY,
    direction VARCHAR(3) NOT NULL,
    description TEXT
);

CREATE TABLE IF NOT EXISTS dates (
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
    date_str TEXT
);

CREATE TABLE IF NOT EXISTS countries (
    country_id SERIAL PRIMARY KEY,
    iso TEXT,
    common_name TEXT,
    name_en TEXT,
    name_fr TEXT,
    prefix TEXT,
    prefix_flag CHAR(1),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT,
    updated_at TIMESTAMP,
    updated_by TEXT
);

CREATE TABLE metrics_type (
    metric_type_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT
);

CREATE TABLE IF NOT EXISTS networks (
    network_id SERIAL PRIMARY KEY,
    tadig TEXT,
    plmn TEXT,
    mcc TEXT,
    mnc TEXT,
    tech_2g TEXT,
    tech_3g TEXT,
    tech_lte TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT,
    updated_at TIMESTAMP,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS operators (
    operator_id SERIAL PRIMARY KEY,
    operator TEXT,
    brand TEXT,
    country_id INT,
    network_id INT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT,
    updated_at TIMESTAMP,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS prefixes (
    prefix_id SERIAL PRIMARY KEY,
    country_id INT,
    operator_id INT,
    cc TEXT,
    ndc TEXT,
    prefix TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT,
    updated_at TIMESTAMP,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS batch_execs (
    batch_id SERIAL PRIMARY KEY,
    batch_name TEXT NOT NULL,
    source_type TEXT,
    source_name TEXT,
    start_time TIMESTAMP,
    end_time TIMESTAMP,        
    corr_id INT,
    batch_status TEXT
);

CREATE TABLE subscribers (
    subscriber_id SERIAL PRIMARY KEY,
    imsi VARCHAR(20) NOT NULL,
    msisdn VARCHAR(20) NOT NULL,
    roam_direction_id INTEGER NOT NULL REFERENCES roam_directions(roam_direction_id),
    first_seen TIMESTAMP,
    last_seen TIMESTAMP    
);

CREATE TABLE metric_definition (
    metric_definition_id SERIAL PRIMARY KEY,
    roam_direction_id INTEGER NOT NULL REFERENCES roam_directions(roam_direction_id),
    metric_type_id INTEGER NOT NULL REFERENCES metrics_type(metric_type_id),  
    name VARCHAR(255) NOT NULL,
    description TEXT,
    is_active BOOLEAN,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT,
    updated_at TIMESTAMP,
    updated_by TEXT    
);

CREATE TABLE metrics (
    metric_id SERIAL PRIMARY KEY,
    metric_definition_id INTEGER NOT NULL REFERENCES metric_definition(metric_definition_id),
    batch_id INTEGER,
    date_id INTEGER NOT NULL REFERENCES dates(date_id),
    country_id INTEGER REFERENCES countries(country_id),
    operator_id INTEGER REFERENCES operators(operator_id),  
    subscriber_id INTEGER REFERENCES subscribers(subscriber_id),  
    value INTEGER 
);

CREATE TABLE IF NOT EXISTS roam_out_perf (
    roam_out_perf_id SERIAL PRIMARY KEY,    
    date_id INT NOT NULL,
    batch_id INT NOT NULL,
    country_id INT,
    operator_id INT,
    country_count INT NOT NULL,
    operator_count INT NOT NULL,
    percent REAL
);

CREATE TABLE IF NOT EXISTS stg_roam_out (
    batch_id INT NOT NULL,
    batch_date TEXT NOT NULL,
    imsi TEXT NOT NULL,
    msisdn TEXT NOT NULL,
    vlr_number TEXT NOT NULL,
    prefix TEXT,
    country_id INT,
    operator_id INT
);

CREATE TABLE IF NOT EXISTS stg_roam_in (
    batch_id INT NOT NULL,
    batch_date TEXT NOT NULL,
    hlraddr TEXT,
    nsub INT,
    nsuba INT,
    prefix TEXT,
    country_id INT,
    operator_id INT
);

CREATE TABLE IF NOT EXISTS sor_plan_config (
    id SERIAL PRIMARY KEY,
    country_id INT NOT NULL,
    operator_id INT NOT NULL,
    rate TEXT,
    routage TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT,
    updated_at TIMESTAMP,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS rules (
    id SERIAL PRIMARY KEY,
    name TEXT,
    description TEXT,
    is_active BOOLEAN,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT,
    updated_at TIMESTAMPTZ,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS notifications (
    id SERIAL PRIMARY KEY,
    date_id INT,
    batch_id INT,
    rule_id INT,
    ref_id INT,
    message TEXT
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

CREATE TABLE IF NOT EXISTS load_sor_plan (
    id SERIAL PRIMARY KEY,
    country TEXT,
    operator TEXT,
    rate TEXT,
    routage TEXT,
    created_by TEXT DEFAULT 'system'
);

COPY countries (iso,common_name,name_en,name_fr,prefix,prefix_flag)
FROM '/countries.csv'
DELIMITER ',' CSV HEADER;

COPY load_operators (tadig, plmn, mcc, mnc, t2g, t3g, lte, operator, brand, country_iso)
FROM '/operators.csv'
DELIMITER ',' CSV HEADER;

COPY load_prefixes (country, operator, cc, ndc, prefix)
FROM '/prefixes.csv'
DELIMITER ',' CSV HEADER;

COPY load_sor_plan (country, operator, rate, routage)
FROM '/sor_plan.csv'
DELIMITER ',' CSV HEADER;

INSERT INTO dates (
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

INSERT INTO global_config (key,value) VALUES ('home_country','Tunisia');
INSERT INTO global_config (key,value) VALUES ('home_operator','Orange');
INSERT INTO roam_directions (direction, description) 
VALUES 
    ('IN', 'ROAM IN'),
    ('OUT', 'ROAM OUT');
INSERT INTO metrics_type (name, description) 
VALUES 
    ('GLOBAL', 'GLOBAL'),
    ('COUNTRY', 'COUNTRY'),
    ('OPERATOR', 'OPERATOR'),
    ('SUBSCRIBER', 'SUBSCRIBER');
UPDATE countries SET created_by = 'system';

INSERT INTO networks (tadig, plmn, mcc, mnc, tech_2g, tech_3g, tech_lte, created_by)
SELECT tadig, plmn, mcc, mnc, t2g, t3g, lte, created_by
FROM load_operators;

INSERT INTO operators (operator, brand, country_id, network_id, created_by)
SELECT ldr.operator, ldr.brand, c.country_id, n.network_id, ldr.created_by
FROM load_operators ldr
JOIN countries c ON ldr.country_iso = c.iso
JOIN networks n ON ldr.tadig = n.tadig;

INSERT INTO prefixes (country_id, cc, prefix, created_by)
SELECT 
  country_id, 
  REPLACE(COALESCE(prefix_item, ''), '-', '') AS cc,  
  REPLACE(COALESCE(prefix_item, ''), '-', '') AS prefix,
  'system'
FROM 
  countries,
  unnest(string_to_array(prefix, ',')) AS prefix_item
  WHERE prefix_flag = 'X';

INSERT INTO prefixes (country_id,operator_id, cc, ndc, prefix, created_by)
SELECT cnt.country_id, opr.operator_id, ldr.cc, ldr.ndc, ldr.prefix, ldr.created_by
FROM load_prefixes ldr
JOIN operators opr ON opr.operator = ldr.operator
JOIN countries cnt ON cnt.common_name = ldr.country
WHERE opr.country_id = cnt.country_id;

DELETE FROM prefixes
WHERE prefix_id IN (
    SELECT prefix_id
    FROM (
        SELECT prefix_id,
               ROW_NUMBER() OVER (PARTITION BY prefix ORDER BY prefix_id) AS rn
        FROM prefixes
    ) t
    WHERE t.rn > 1
);

insert INTO sor_plan_config (country_id, operator_id, rate, routage, created_by)
SELECT ope.country_id, ope.operator_id, fct.rate, fct.routage, fct.created_by
from load_sor_plan fct JOIN countries ctn ON fct.country = ctn.common_name
JOIN operators ope ON fct.operator = ope.operator
WHERE ctn.country_id = ope.country_id;

-- DROP TABLE load_operators;
-- DROP TABLE load_prefixes;
-- DROP TABLE load_sor_plan;

----------------------
-- Business
----------------------
INSERT INTO metric_definition (roam_direction_id, metric_type_id, name, description, is_active)
SELECT 
    rd.roam_direction_id,
    mt.metric_type_id,
    'number_subscribers_in',
    'Number of subscribers IN',
    TRUE
FROM roam_directions rd
CROSS JOIN metrics_type mt
WHERE rd.direction = 'IN' AND mt.name = 'GLOBAL';

INSERT INTO metric_definition (roam_direction_id, metric_type_id, name, description, is_active)
SELECT 
    rd.roam_direction_id,
    mt.metric_type_id,
    'number_subscribers_in_by_country',
    'Number of subscribers IN (By country)',
    TRUE
FROM roam_directions rd
CROSS JOIN metrics_type mt
WHERE rd.direction = 'IN' AND mt.name = 'COUNTRY';

INSERT INTO metric_definition (roam_direction_id, metric_type_id, name, description, is_active)
SELECT 
    rd.roam_direction_id,
    mt.metric_type_id,
    'number_subscribers_in_by_operator',
    'Number of subscribers IN (By operator)',
    TRUE
FROM roam_directions rd
CROSS JOIN metrics_type mt
WHERE rd.direction = 'IN' AND mt.name = 'OPERATOR';

INSERT INTO metric_definition (roam_direction_id, metric_type_id, name, description, is_active)
SELECT 
    rd.roam_direction_id,
    mt.metric_type_id,
    'number_active_subscribers_in',
    'Number of active subscribers IN',
    TRUE
FROM roam_directions rd
CROSS JOIN metrics_type mt
WHERE rd.direction = 'IN' AND mt.name = 'GLOBAL';

INSERT INTO metric_definition (roam_direction_id, metric_type_id, name, description, is_active)
SELECT 
    rd.roam_direction_id,
    mt.metric_type_id,
    'number_active_subscribers_in_by_country',
    'Number of active subscribers IN (By country)',
    TRUE
FROM roam_directions rd
CROSS JOIN metrics_type mt
WHERE rd.direction = 'IN' AND mt.name = 'COUNTRY';

INSERT INTO metric_definition (roam_direction_id, metric_type_id, name, description, is_active)
SELECT 
    rd.roam_direction_id,
    mt.metric_type_id,
    'number_active_subscribers_in_by_operator',
    'Number of active subscribers IN (By operator)',
    TRUE
FROM roam_directions rd
CROSS JOIN metrics_type mt
WHERE rd.direction = 'IN' AND mt.name = 'OPERATOR';

INSERT INTO metric_definition (roam_direction_id, metric_type_id, name, description, is_active)
SELECT 
    rd.roam_direction_id,
    mt.metric_type_id,
    'number_subscribers_out',
    'Number of subscribers OUT',
    TRUE
FROM roam_directions rd
CROSS JOIN metrics_type mt
WHERE rd.direction = 'OUT' AND mt.name = 'GLOBAL';

INSERT INTO metric_definition (roam_direction_id, metric_type_id, name, description, is_active)
SELECT 
    rd.roam_direction_id,
    mt.metric_type_id,
    'number_subscribers_out_by_country',
    'Number of subscribers OUT (By country)',
    TRUE
FROM roam_directions rd
CROSS JOIN metrics_type mt
WHERE rd.direction = 'OUT' AND mt.name = 'COUNTRY';

INSERT INTO metric_definition (roam_direction_id, metric_type_id, name, description, is_active)
SELECT 
    rd.roam_direction_id,
    mt.metric_type_id,
    'number_subscribers_out_by_operator',
    'Number of subscribers OUT (By operator)',
    TRUE
FROM roam_directions rd
CROSS JOIN metrics_type mt
WHERE rd.direction = 'OUT' AND mt.name = 'OPERATOR';

INSERT INTO rules (name , description, is_active) VALUES ('imsi_is_not_local','IMSI non local',TRUE);
INSERT INTO rules (name , description, is_active) VALUES ('local_vlr_number','vlr_number Local ',TRUE);
INSERT INTO rules (name , description, is_active) VALUES ('sor_plan_bar','Barring operator',TRUE);
INSERT INTO rules (name , description, is_active) VALUES ('sor_plan_deviation','Deviation SoR',TRUE);

----------------------
-- Indexes
----------------------
CREATE INDEX idx_metrics_i1 ON metrics (date_id,country_id,operator_id);

----------------------
-- Views
----------------------
create view v_metrics as
select ms.name as metric_type,rd.direction, md.name as metric_name, md.description as metric_description, pr.date_str,cn.common_name as country, op.operator, mt.value  
from metrics mt 
join metric_definition md on mt.metric_definition_id = md.metric_definition_id
join dates pr on pr.date_id = mt.date_id
join metrics_type ms on ms.metric_type_id = md.metric_type_id
left join countries cn on cn.country_id = mt.country_id
left join operators op on op.operator_id = mt.operator_id
left join roam_directions rd on md.roam_direction_id = rd.roam_direction_id;


CREATE OR REPLACE VIEW v_roam_out_perf AS
    SELECT 
        agg.date_id, 
        agg.batch_id, 
        (SELECT id FROM rules WHERE name = 'sor_plan_deviation') AS rule_id, 
        agg.roam_out_perf_id AS ref_id,
        CAST(pln.rate AS FLOAT) AS perct_configure,
        agg.percent AS perct_reel,
        ope.operator,
        cnt.common_name
    FROM (
        SELECT *
        FROM roam_out_perf fct
        WHERE country_id IN (SELECT country_id FROM sor_plan_config)
    ) agg
    LEFT JOIN sor_plan_config pln 
        ON agg.country_id = pln.country_id 
       AND agg.operator_id = pln.operator_id
    JOIN operators ope 
        ON pln.operator_id = ope.operator_id
    JOIN countries cnt ON agg.country_id = cnt.country_id 
    ORDER by cnt.common_name, ope.operator;