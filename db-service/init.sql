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
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
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
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
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
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_by TEXT
);

-- Create networks_technologies junction table
CREATE TABLE IF NOT EXISTS networks_technologies (
    id SERIAL PRIMARY KEY,
    network_id INTEGER,
    technology_id INTEGER,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
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
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
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

-- Seed technologies
INSERT INTO technologies (name, created_by) VALUES 
    ('2G', 'system'),
    ('3G', 'system'),
    ('LTE', 'system');

-- Seed countries
INSERT INTO countries (designation, name, iso, created_by) VALUES 
    ('France', 'France', 'FR', 'system'),
    ('French Antilles', 'French Antilles', 'BL/GF/GP/MF/MQ', 'system'),
    ('French Polynesia', 'French Polynesia', 'PF', 'system'),
    ('New Caledonia', 'New Caledonia', 'NC', 'system'),
    ;

-- Load raw operator data
INSERT INTO load_operators (tadig, plmn, mcc, mnc, t2g, t3g, lte, operator, brand, country_iso) VALUES 
    ('FRAF1', '20801', '208', '01', 'Yes', 'Yes', 'Yes', 'Orange', 'Orange S.A.', 'FR'),
    ('FRAF2', '20810', '208', '10', 'Yes', 'Yes', 'Yes', 'SFR', 'Altice', 'FR'),
    ('FRAF3', '20820', '208', '20', 'Yes', 'Yes', 'Yes', 'Bouygues', 'Bouygues Telecom', 'FR'),
    ('FRAF4', '34020', '340', '20', 'Yes', 'Yes', 'Yes', 'Digicel', 'DIGICEL Antilles Française Guyane', 'BL/GF/GP/MF/MQ'),
    ('FRAFM', '20815', '208', '15', 'Yes', 'Yes', 'Yes', 'Free', 'Iliad', 'FR'),
    ('FRATK', '54720', '547', '20', 'Yes', 'Yes', 'Yes', 'Vini', 'Onati S.A.S.', 'PF'),
    ('GLP01', '34001', '340', '01', 'Yes', 'Yes', 'Yes', 'Orange', 'Orange Caraïbe Mobiles', 'BL/GF/GP/MF/MQ'),
    ('NCLPT', '54601', '546', '01', 'Yes', 'Yes', 'Yes', 'Mobilis', 'OPT New Caledonia', 'NC');    

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
