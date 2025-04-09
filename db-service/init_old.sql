-- Drop tables if they exist
DROP TABLE IF EXISTS operators;
DROP TABLE IF EXISTS networks_technologies;
DROP TABLE IF EXISTS networks;
DROP TABLE IF EXISTS technologies;
DROP TABLE IF EXISTS countries;
DROP TABLE IF EXISTS load_operators;

-- Create technologies table
CREATE TABLE IF NOT EXISTS technologies (
    id SERIAL PRIMARY KEY,
    name TEXT,
    bands TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT,
    updated_at TIMESTAMPTZ,
    updated_by TEXT
);

-- Create countries table
CREATE TABLE IF NOT EXISTS countries (
    id SERIAL PRIMARY KEY,
    designation TEXT,
    name TEXT,
    iso TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT,
    updated_at TIMESTAMPTZ,
    updated_by TEXT
);

-- Create networks table
CREATE TABLE IF NOT EXISTS networks (
    id SERIAL PRIMARY KEY,
    tadig TEXT,
    plmn TEXT,
    mcc TEXT,
    mnc TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT,
    updated_at TIMESTAMPTZ,
    updated_by TEXT
);

-- Create networks_technologies junction table
CREATE TABLE IF NOT EXISTS networks_technologies (
    id SERIAL PRIMARY KEY,
    network_id INTEGER,
    technology_id INTEGER,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT,
    updated_at TIMESTAMPTZ,
    updated_by TEXT,
    
    CONSTRAINT fk_technologies
        FOREIGN KEY (technology_id)
        REFERENCES technologies(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_networks
        FOREIGN KEY (network_id)
        REFERENCES networks(id)
        ON DELETE CASCADE
);

-- Create operators table
CREATE TABLE IF NOT EXISTS operators (
    id SERIAL PRIMARY KEY,
    operator TEXT,
    brand TEXT,
    country_id INTEGER NOT NULL,
    network_id INTEGER NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT,
    updated_at TIMESTAMPTZ,
    updated_by TEXT,

    CONSTRAINT fk_countries
        FOREIGN KEY (country_id)
        REFERENCES countries(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_networks
        FOREIGN KEY (network_id)
        REFERENCES networks(id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS prefixes (
    id SERIAL PRIMARY KEY,
    prefix TEXT,
    cc TEXT,
    ndc TEXT,        
    operator_id INTEGER NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT,
    updated_at TIMESTAMPTZ,
    updated_by TEXT,

    CONSTRAINT fk_operators
        FOREIGN KEY (operator_id)
        REFERENCES operators(id)
        ON DELETE CASCADE
);

CREATE TABLE dim_time (
    id SERIAL PRIMARY KEY,
    date DATE NOT NULL,
    year INT NOT NULL,
    quarter INT NOT NULL,
    month INT NOT NULL,
    day INT NOT NULL,
    day_of_week INT NOT NULL,
    day_name TEXT NOT NULL,
    week_of_year INT NOT NULL,
    is_weekend BOOLEAN NOT NULL,
    date_text TEXT -- Text format YYYY-MM-DD
);

CREATE INDEX idx_dim_time
ON dim_time (date_text);

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

CREATE TABLE IF NOT EXISTS stg_roam_out (
    batch_id INT NOT NULL,
    batch_date TEXT  NOT NULL,      
    imsi TEXT NOT NULL,
    msisdn TEXT NOT NULL,
    vlr_number TEXT NOT NULL,
    carrier_name TEXT,   
    country_name TEXT,
    country_iso TEXT 
);

CREATE INDEX idx_stg_roam_out
ON stg_roam_out (batch_id);


CREATE TABLE IF NOT EXISTS dim_carriers (
    id SERIAL PRIMARY KEY,  
    country_name TEXT,
    carrier_name TEXT, 
    country_iso TEXT     
);

CREATE TABLE IF NOT EXISTS dim_imsi (
    id SERIAL PRIMARY KEY,  
    imsi TEXT NOT NULL    
);

CREATE INDEX idx_dim_imsi
ON dim_imsi (imsi);

CREATE TABLE IF NOT EXISTS dim_msisdn (
    id SERIAL PRIMARY KEY,  
    msisdn TEXT NOT NULL    
);

CREATE INDEX idx_dim_msisdn
ON dim_msisdn (msisdn);

CREATE TABLE IF NOT EXISTS fct_roam_out (
    batch_id INT NOT NULL,
    date_id INT NOT NULL,      
    imsi_id INT NOT NULL,
    msisdn_id INT NOT NULL,
    carrier_id INT NOT NULL
);

-- Create load_operators staging table
CREATE TABLE IF NOT EXISTS load_operators (
    id SERIAL PRIMARY KEY,
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
    created_by TEXT DEFAULT 'system'
);

CREATE TABLE IF NOT EXISTS load_prefixes (
    id SERIAL PRIMARY KEY,
    country TEXT,
    carrier_name TEXT,
    CC TEXT,
    NDC TEXT,
    prefix TEXT,                       
    created_by TEXT DEFAULT 'system'
);


-- Load CSV
COPY countries(designation, name, iso,created_by)
FROM '/countries.csv'
DELIMITER ','
CSV HEADER;

COPY load_operators(tadig, plmn, mcc, mnc, t2g, t3g, lte, operator, brand, country_iso)
FROM '/operators.csv'
DELIMITER ','
CSV HEADER;

COPY load_prefixes(country, carrier_name, cc, ndc,prefix)
FROM '/prefixes.csv'
DELIMITER ','
CSV HEADER;


-- Seed technologies
INSERT INTO technologies (name, created_by) VALUES 
    ('2G', 'system'),
    ('3G', 'system'),
    ('LTE', 'system');

-- Insert into networks from load_operators
INSERT INTO networks (tadig, plmn, mcc, mnc, created_by)
SELECT tadig, plmn, mcc, mnc, created_by
FROM load_operators;

-- Map technologies to networks
-- 2G
INSERT INTO networks_technologies (network_id, technology_id, created_by)
SELECT 
    net.id, tech.id, ldr.created_by
FROM 
    networks net
JOIN 
    load_operators ldr ON net.tadig = ldr.tadig
JOIN 
    technologies tech ON tech.name = '2G'
WHERE 
    ldr.t2g = 'Yes';

-- 3G
INSERT INTO networks_technologies (network_id, technology_id, created_by)
SELECT 
    net.id, tech.id, ldr.created_by
FROM 
    networks net
JOIN 
    load_operators ldr ON net.tadig = ldr.tadig
JOIN 
    technologies tech ON tech.name = '3G'
WHERE 
    ldr.t3g = 'Yes';

-- LTE
INSERT INTO networks_technologies (network_id, technology_id, created_by)
SELECT 
    net.id, tech.id, ldr.created_by
FROM 
    networks net
JOIN 
    load_operators ldr ON net.tadig = ldr.tadig
JOIN 
    technologies tech ON tech.name = 'LTE'
WHERE 
    ldr.lte = 'Yes';

-- Insert final operator records
INSERT INTO operators (operator, brand, country_id, network_id, created_by)
SELECT 
    ldr.operator, 
    ldr.brand, 
    cnt.id, 
    net.id, 
    ldr.created_by
FROM 
    load_operators ldr 
JOIN 
    countries cnt ON ldr.country_iso = cnt.iso 
JOIN 
    networks net ON net.tadig = ldr.tadig;


INSERT INTO prefixes (prefix,cc,ndc,operator_id, created_by)
Select ldr.prefix, ldr.cc, ldr.ndc, opr.id , ldr.created_by
from load_prefixes ldr
JOIN operators opr ON  opr.operator = ldr.carrier_name
JOIN countries cnt ON cnt.name  = ldr.country
WHERE opr.country_id = cnt.id;


DELETE FROM prefixes
WHERE prefix IN (
    SELECT prefix FROM (
        SELECT prefix,
               ROW_NUMBER() OVER (PARTITION BY prefix ORDER BY id) AS rn
        FROM prefixes
    ) t
    WHERE t.rn > 1
);

INSERT INTO dim_time (
    date, year, quarter, month, day, day_of_week, day_name, week_of_year, is_weekend, date_text
)
SELECT 
    d::date AS date,
    EXTRACT(YEAR FROM d) AS year,
    EXTRACT(QUARTER FROM d) AS quarter,
    EXTRACT(MONTH FROM d) AS month,
    EXTRACT(DAY FROM d) AS day,
    EXTRACT(ISODOW FROM d) AS day_of_week, -- 1 = Monday, 7 = Sunday
    TO_CHAR(d, 'Day') AS day_name,
    EXTRACT(WEEK FROM d) AS week_of_year,
    CASE WHEN EXTRACT(ISODOW FROM d) IN (6, 7) THEN TRUE ELSE FALSE END AS is_weekend,
    TO_CHAR(d, 'YYYY-MM-DD') AS date_text
FROM GENERATE_SERIES((DATE_TRUNC('year', NOW()) - INTERVAL '1 year')::DATE, (DATE_TRUNC('year', NOW()) + INTERVAL '5 years - 1 day')::DATE, '1 day'::INTERVAL) AS d;





--------------------
-- WORK
--------------------

CREATE TABLE IF NOT EXISTS streeing_config (
    id SERIAL PRIMARY KEY,  
    operator_id INTEGER NOT NULL, 
    rate TEXT 
);


INSERT INTO streeing_config (operator_id, rate) VALUES (1,'85'); -- orange
INSERT INTO streeing_config (operator_id, rate) VALUES (2,''); -- SFR
INSERT INTO streeing_config (operator_id, rate) VALUES (3,''); -- Bouyges
INSERT INTO streeing_config (operator_id, rate) VALUES (5,'15'); -- Free
