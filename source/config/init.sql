-------------------------------------------
-- REFERENCE 
-------------------------------------------

CREATE TABLE IF NOT EXISTS ref_global_config (
    global_config_id SERIAL PRIMARY KEY,
    key VARCHAR(100),
    value VARCHAR(100) 
);

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

INSERT INTO ref_global_config (key, value)
VALUES ('home_country', 'Tunisia'),
       ('home_operator', 'Orange'),
       ('deviance_interval', '2')
       ;

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
    ('sor_plan_deviation', 'Deviation SoR'),
    ('baring_operator', 'Baring Operator');

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
    last_seen TIMESTAMP,
    CONSTRAINT unique_imsi UNIQUE (imsi)    
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
    target_percentage DECIMAL(10, 2),
    actual_percentage DECIMAL(10, 2),
    is_outside_tolerance BOOLEAN DEFAULT FALSE,
    is_barring BOOLEAN DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS trx_notifications (
    notification_id SERIAL PRIMARY KEY,
    batch_id INTEGER NOT NULL REFERENCES batch_execs(batch_id), 
    date_id INTEGER NOT NULL REFERENCES ref_dates(date_id),
    rule_id INTEGER NOT NULL REFERENCES ref_rules(rule_id),
    ref_id INT,
    message TEXT NULL
);


CREATE TABLE IF NOT EXISTS trx_alerts (
    alert_id SERIAL PRIMARY KEY,
    batch_id INTEGER NOT NULL REFERENCES batch_execs(batch_id), 
    date_id INTEGER NOT NULL REFERENCES ref_dates(date_id),
    rule_id INTEGER NOT NULL REFERENCES ref_rules(rule_id),
    ref_id INT,
    message TEXT NULL
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
-- USERS
-------------------------------------------
-- Enhanced users table with roles
CREATE TYPE user_role AS ENUM ('super_admin', 'admin', 'operator', 'viewer');

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role user_role DEFAULT 'viewer',
    is_active BOOLEAN DEFAULT true,
    permissions JSONB DEFAULT '[]',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create user_permissions table for fine-grained permissions
CREATE TABLE user_permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    permission_key VARCHAR(100) NOT NULL,
    granted BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, permission_key)
);

CREATE INDEX idx_users_username_lower_active ON users (LOWER(username), is_active);
CREATE INDEX idx_users_email_lower ON users (LOWER(email));
CREATE INDEX idx_user_permissions_user_granted ON user_permissions (user_id, granted);
CREATE INDEX idx_users_id_active ON users (id, is_active);

-- Insert default users with different roles
INSERT INTO users (username, email, password_hash, role) VALUES 
    ('superadmin', 'superadmin@roamadmin.com', '$2b$12$1MNfH3KR0knWJBQk.YZ5MeXANUmvzGvj52j2RkroK8s/Tu2Dns7VG', 'super_admin'), -- password: superadmin123
    ('admin', 'admin@roamadmin.com', '$2b$12$PxvAb1.BKewkD5lgGThncueBC8V33Y7NWnTkFUHWtgHp47a7cClO.', 'admin'), -- password: admin123
    ('operator1', 'operator1@roamadmin.com', '$2b$12$u5P2fCYeHvzCE0AE2Y5AIePKQiS7aSpjF87s1sjVPAHeRwjGbeOmu', 'operator'), -- password: operator123
    ('viewer1', 'viewer1@roamadmin.com', '$2b$12$QJUTmtIm9Ur/p5HoNoTDfuH1QvpbBWfCVhgGlZtcw9xtYUOT1fDCC', 'viewer'); -- password: viewer123

-- Insert permissions for different roles
INSERT INTO user_permissions (user_id, permission_key) 
SELECT id, unnest(ARRAY[
    'dashboard:read',
    'roamin:read',
    'roamout:read',
    'sorperformance:read'
]) FROM users WHERE role = 'viewer';

INSERT INTO user_permissions (user_id, permission_key) 
SELECT id, unnest(ARRAY[
    'dashboard:read',
    'roamin:read', 'roamin:write',
    'roamout:read', 'roamout:write', 
    'sorperformance:read', 'sorperformance:write',
    'countries:read',
    'operators:read'
]) FROM users WHERE role = 'operator';

INSERT INTO user_permissions (user_id, permission_key) 
SELECT id, unnest(ARRAY[
    'dashboard:read',
    'roamin:read', 'roamin:write', 'roamin:delete',
    'roamout:read', 'roamout:write', 'roamout:delete',
    'sorperformance:read', 'sorperformance:write', 'sorperformance:delete',
    'countries:read', 'countries:write', 'countries:delete',
    'operators:read', 'operators:write', 'operators:delete',
    'sorplans:read', 'sorplans:write', 'sorplans:delete',
    'prefixes:read', 'prefixes:write', 'prefixes:delete',
    'users:read', 'users:write'
]) FROM users WHERE role = 'admin';

INSERT INTO user_permissions (user_id, permission_key) 
SELECT id, unnest(ARRAY[
    'dashboard:read',
    'roamin:read', 'roamin:write', 'roamin:delete',
    'roamout:read', 'roamout:write', 'roamout:delete',
    'sorperformance:read', 'sorperformance:write', 'sorperformance:delete',
    'countries:read', 'countries:write', 'countries:delete',
    'operators:read', 'operators:write', 'operators:delete',
    'sorplans:read', 'sorplans:write', 'sorplans:delete',
    'prefixes:read', 'prefixes:write', 'prefixes:delete',
    'users:read', 'users:write', 'users:delete',
    'system:admin'
]) FROM users WHERE role = 'super_admin';
-------------------------------------------
-- VIEWS
-------------------------------------------
CREATE OR REPLACE VIEW v_metrics_global AS
SELECT 
  trx.batch_id,
  rd.date,
  rrd.direction,
  rmt.name as metric_type,
  trx.value
FROM 
  trx_metrics_global trx
  JOIN ref_dates rd ON trx.date_id = rd.date_id
  JOIN ref_metric_definitions rmd ON trx.metric_definition_id = rmd.metric_definition_id
  JOIN ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id
  JOIN ref_metric_types rmt ON rmd.metric_type_id = rmt.metric_type_id;

CREATE OR REPLACE VIEW v_metrics_country AS
SELECT 
  trx.batch_id,
  rd.date,
  rrd.direction,
  rmt.name as metric_type,
  cc.country_name,
  trx.value
FROM 
  trx_metrics_country trx
  JOIN ref_dates rd ON trx.date_id = rd.date_id
  JOIN ref_metric_definitions rmd ON trx.metric_definition_id = rmd.metric_definition_id
  JOIN ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id
  JOIN ref_metric_types rmt ON rmd.metric_type_id = rmt.metric_type_id
  LEFT JOIN cfg_countries cc ON cc.country_id = trx.country_id
  ;

CREATE OR REPLACE VIEW v_metrics_operator AS
SELECT 
  trx.batch_id,
  rd.date,
  rrd.direction,
  rmt.name as metric_type,
  cc.country_name,
  co.operator_name,  
  trx.value
FROM 
  trx_metrics_operator trx
  JOIN ref_dates rd ON trx.date_id = rd.date_id
  JOIN ref_metric_definitions rmd ON trx.metric_definition_id = rmd.metric_definition_id
  JOIN ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id
  JOIN ref_metric_types rmt ON rmd.metric_type_id = rmt.metric_type_id
  LEFT JOIN cfg_operators co ON co.operator_id = trx.operator_id
  LEFT JOIN cfg_countries cc ON cc.country_id = co.country_id  
  ;  
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


CREATE TABLE IF NOT EXISTS ldr_sor_plan (
    id SERIAL PRIMARY KEY,
    country TEXT,
    operator TEXT,
    rate TEXT,
    routage TEXT,
    baring TEXT,
    created_by TEXT DEFAULT 'system'
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

COPY ldr_sor_plan (country, operator, rate, routage, baring)
FROM '/sor_plan.csv'
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

INSERT INTO cfg_sor_plan (operator_id, rate, barring, created_by, routage_type_id)
SELECT 
  o.operator_id, 
  ldr.rate, 
  CASE WHEN ldr.baring = 'X' THEN TRUE ELSE FALSE END AS barring,
  ldr.created_by,
  r.routage_type_id
FROM 
  ldr_sor_plan ldr
  JOIN cfg_countries c ON upper(ldr.country) = upper(c.country_name)
  JOIN cfg_operators o ON upper(ldr.operator) = upper(o.operator_name)
  LEFT JOIN ref_routage_types r ON upper(ldr.routage) = upper(r.routage_type_name)
WHERE 
  c.country_id = o.country_id;

DROP TABLE ldr_countries;
DROP TABLE ldr_operators;
DROP TABLE ldr_prefixes;
DROP TABLE ldr_sor_plan;


