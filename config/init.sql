-----------------------------------------------------------------------
-- Config
-----------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS global_config (
    global_config_id SERIAL PRIMARY KEY,
    key TEXT,
    value TEXT
);

CREATE TABLE directions (
    direction_id SERIAL PRIMARY KEY,
    name VARCHAR(3) NOT NULL,
    description TEXT
);

CREATE TABLE technologies (
    technology_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description VARCHAR(255)
);

INSERT INTO global_config (key, value) VALUES('home_country','Tunisia');
INSERT INTO global_config (key, value) VALUES('home_operator','Orange');
INSERT INTO directions (name, description) VALUES('IN','Inbound');
INSERT INTO directions (name, description) VALUES('OUT','Outbound');
INSERT INTO technologies (name, description) VALUES('2G','2G');
INSERT INTO technologies (name, description) VALUES('3G','3G');
INSERT INTO technologies (name, description) VALUES('LTE','LTE');

-----------------------------------------------------------------------
-- Config
-----------------------------------------------------------------------

CREATE TABLE countries (
    country_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    name_en VARCHAR(100) NOT NULL,
    name_fr VARCHAR(100) NOT NULL,
    iso VARCHAR(20),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100),
    updated_at TIMESTAMP,
    updated_by VARCHAR(100),
    CONSTRAINT uq_country_name UNIQUE (name)
);

CREATE TABLE operators (
    operator_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    short_name VARCHAR(50),
    website VARCHAR(255),
    country_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100),
    updated_at TIMESTAMP,
    updated_by VARCHAR(100),
    FOREIGN KEY (country_id) REFERENCES countries(country_id) 
);

CREATE TABLE networks (
    network_id SERIAL PRIMARY KEY,
    plmn VARCHAR(6) NOT NULL,    
    mcc VARCHAR(3) NOT NULL,
    mnc VARCHAR(3) NOT NULL,
    tadig VARCHAR(10),
    operator_id INT NOT NULL, 
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100),
    updated_at TIMESTAMP,
    updated_by VARCHAR(100),
    FOREIGN KEY (operator_id) REFERENCES operators(operator_id)
);

CREATE TABLE network_technologies (
    network_technology_id SERIAL PRIMARY KEY,
    network_id INT NOT NULL,
    technology_id INT NOT NULL,
    is_primary BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100),
    updated_at TIMESTAMP,
    updated_by VARCHAR(100),
    FOREIGN KEY (network_id) REFERENCES networks(network_id),
    FOREIGN KEY (technology_id) REFERENCES technologies(technology_id)
);

CREATE TABLE prefixes (
    prefix_id SERIAL PRIMARY KEY,
    prefix VARCHAR(10) NOT NULL,
    country_id INT,
    operator_id INT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100),
    updated_at TIMESTAMP,
    updated_by VARCHAR(100),
    FOREIGN KEY (country_id) REFERENCES countries(country_id),
    FOREIGN KEY (operator_id) REFERENCES operators(operator_id)
);

-- CREATE INDEX idx_prefixes_country ON prefixes(country_id);
-- CREATE INDEX idx_prefixes_operator ON prefixes(operator_id);
-- CREATE INDEX idx_networks_operator ON networks(operator_id);
-- CREATE INDEX idx_operators_country ON operators(country_id);


-----------------------------------------------------------------------
-- Load config: countries
-----------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS load_countries (
    iso TEXT,
    common_name TEXT,
    name_en TEXT,
    name_fr TEXT,
    prefix TEXT,
    prefix_flag CHAR(1),
    created_by   TEXT DEFAULT 'system'
);

COPY load_countries (iso,common_name,name_en,name_fr,prefix,prefix_flag)
FROM '/countries.csv'
DELIMITER ',' CSV HEADER;


INSERT INTO countries (name, name_en, name_fr, iso)
SELECT common_name, name_en, name_fr, iso
FROM load_countries;

INSERT INTO prefixes (prefix, country_id, created_by)
SELECT 
    TRIM(REPLACE(COALESCE(prefix_item, ''), '-', '')) AS prefix, 
    cnt.country_id,
    ldr.created_by
FROM 
    countries cnt 
JOIN 
    load_countries ldr ON cnt.name = ldr.common_name
CROSS JOIN 
    unnest(string_to_array(ldr.prefix, ',')) AS prefix_item
WHERE 
    ldr.prefix_flag = 'X'
    AND TRIM(REPLACE(COALESCE(prefix_item, ''), '-', '')) != '';

DROP TABLE load_countries;


-----------------------------------------------------------------------
-- XXXXX
-----------------------------------------------------------------------
