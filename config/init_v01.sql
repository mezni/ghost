CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
----------------------
-- Users
----------------------
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    first_name VARCHAR(50),
    last_name VARCHAR(50),
    is_valid BOOLEAN DEFAULT FALSE,
    is_admin BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_users_updated_at
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

----------------------
-- Config
----------------------
CREATE TABLE routage_types (
    routage_type_id SERIAL PRIMARY KEY,
    routage_type_name VARCHAR(100) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100) NOT NULL,
    updated_at TIMESTAMP NULL,
    updated_by VARCHAR(100) NULL
);

CREATE TABLE technology_statuses (
    technology_status_id SERIAL PRIMARY KEY,
    technology_status_name VARCHAR(100) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100) NOT NULL,
    updated_at TIMESTAMP NULL,
    updated_by VARCHAR(100) NULL
);


CREATE TABLE IF NOT EXISTS countries (
    country_id SERIAL PRIMARY KEY,
    iso_code VARCHAR(100) NOT NULL UNIQUE,
    country_name VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100) NOT NULL,
    updated_at TIMESTAMP NULL,
    updated_by VARCHAR(100) NULL
);

CREATE TABLE IF NOT EXISTS operators (
    operator_id SERIAL PRIMARY KEY,
    operator_name VARCHAR(100) NOT NULL,
    country_id INTEGER REFERENCES countries(country_id), 
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100) NOT NULL,
    updated_at TIMESTAMP NULL,
    updated_by VARCHAR(100) NULL
);

CREATE TABLE IF NOT EXISTS networks (
    network_id SERIAL PRIMARY KEY,
    plmn_code VARCHAR(100) NOT NULL,
    plmn VARCHAR(100) NOT NULL,
    mcc VARCHAR(100) NOT NULL,
    mnc VARCHAR(100) NOT NULL,
    operator_id INTEGER REFERENCES operators(operator_id), 
    tech_2g VARCHAR(3),
    tech_3g VARCHAR(3),
    tech_lte VARCHAR(3),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100) NOT NULL,
    updated_at TIMESTAMP NULL,
    updated_by VARCHAR(100) NULL
);

CREATE TABLE IF NOT EXISTS sor_plan (
    sor_plan_id SERIAL PRIMARY KEY,
    operator_id INTEGER REFERENCES operators(operator_id), 
    routage_type_id INTEGER REFERENCES routage_types(routage_type_id),
    barring VARCHAR(1),    
    rate VARCHAR(100),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(100) NOT NULL,
    updated_at TIMESTAMP NULL,
    updated_by VARCHAR(100) NULL,
    is_current BIT NOT NULL,
    version INT NOT NULL
);



create or replace view v_sor_plan as
select net.plmn_code, cnt.country_name, opr.operator_name, sor.barring, sor.rate, net.mcc, net.mnc, routage_type_name, net.tech_2g, net.tech_3g, net.tech_lte
from sor_plan sor 
join operators opr on sor.operator_id = opr.operator_id
join countries cnt on opr.country_id = cnt.country_id
join networks net on opr.operator_id = net.operator_id
join routage_types rtt on sor.routage_type_id = rtt.routage_type_id
where sor.is_current = TRUE;

------------------------ 
-- PARAMETRAGES
------------------------ 


INSERT INTO routage_types (routage_type_name, created_by) VALUES
('Bilateral', 'system'),
('Orange Hub', 'system'),
('Comfone', 'system'),
('N/A', 'system')
;

INSERT INTO technology_statuses (technology_status_name, created_by) VALUES
('Yes', 'system'),
('No', 'system'),
('Stopped', 'system'),
('Planned', 'system'),
('N/A', 'system')
;


------------------------ 
-- TEMPORAIRE
------------------------

INSERT INTO countries (iso_code, country_name, created_by) VALUES
('TN', 'Tunisia', 'system'),
('DZ', 'Algeria', 'system'),
('AT', 'Austria', 'system')
;

INSERT INTO operators (operator_name, country_id, created_by) 
VALUES
('Orange', (SELECT country_id FROM countries WHERE iso_code = 'TN'), 'system'),
('Ooredoo', (SELECT country_id FROM countries WHERE iso_code = 'TN'), 'system'),
('Tunisie Telecom', (SELECT country_id FROM countries WHERE iso_code = 'TN'), 'system'),
('Mobilis', (SELECT country_id FROM countries WHERE iso_code = 'DZ'), 'system'),
('Djezzy', (SELECT country_id FROM countries WHERE iso_code = 'DZ'), 'system'),
('Ooredoo', (SELECT country_id FROM countries WHERE iso_code = 'DZ'), 'system'),
('H3G', (SELECT country_id FROM countries WHERE iso_code = 'AT'), 'system'),
('T-Mobile', (SELECT country_id FROM countries WHERE iso_code = 'AT'), 'system'),
('A1 Telekom', (SELECT country_id FROM countries WHERE iso_code = 'AT'), 'system')
;

INSERT INTO networks (plmn, plmn_code, mcc, mnc, operator_id, tech_2g, tech_3g, tech_lte, created_by) VALUES
('60501', 'TUNOR', '605', '01', (SELECT operator_id FROM operators WHERE country_id = (SELECT country_id FROM countries WHERE iso_code = 'TN') AND operator_name = 'Orange'), 'Yes', 'Yes', 'Yes', 'system'),
('60502', 'TUNTA', '605', '02', (SELECT operator_id FROM operators WHERE country_id = (SELECT country_id FROM countries WHERE iso_code = 'TN') AND operator_name = 'Ooredoo'), 'Yes', 'Yes', 'Yes', 'system'),
('60503', 'TUNTT', '605', '03', (SELECT operator_id FROM operators WHERE country_id = (SELECT country_id FROM countries WHERE iso_code = 'TN') AND operator_name = 'Tunisie Telecom'), 'Yes', 'Yes', 'Yes', 'system'),
('60301', 'DZAA1', '603', '01', (SELECT operator_id FROM operators WHERE country_id = (SELECT country_id FROM countries WHERE iso_code = 'DZ') AND operator_name = 'Mobilis'), 'Yes', 'Yes', 'Yes', 'system'),
('60302', 'DZAOT', '603', '02', (SELECT operator_id FROM operators WHERE country_id = (SELECT country_id FROM countries WHERE iso_code = 'DZ') AND operator_name = 'Djezzy'), 'Yes', 'Yes', 'Yes', 'system'),
('60303', 'DZAWT', '603', '03', (SELECT operator_id FROM operators WHERE country_id = (SELECT country_id FROM countries WHERE iso_code = 'DZ') AND operator_name = 'Ooredoo'), 'Yes', 'Yes', 'Yes', 'system'),
('23205', 'AUTCA', '232', '05', (SELECT operator_id FROM operators WHERE country_id = (SELECT country_id FROM countries WHERE iso_code = 'AT') AND operator_name = 'H3G'), 'Yes', 'Yes', 'Yes', 'system'),
('23203', 'AUTMM', '232', '03', (SELECT operator_id FROM operators WHERE country_id = (SELECT country_id FROM countries WHERE iso_code = 'AT') AND operator_name = 'T-Mobile'), 'Yes', 'Yes', 'Yes', 'system'),
('23201', 'AUTPT', '232', '01', (SELECT operator_id FROM operators WHERE country_id = (SELECT country_id FROM countries WHERE iso_code = 'AT') AND operator_name = 'A1 Telekom'), 'Yes', 'Yes', 'Yes', 'system')
;


INSERT INTO sor_plan (operator_id, routage_type_id, rate, version, created_by) VALUES
(4, 1, '90', 1 , 'system'),
(5, 1, '10', 1 , 'system'),
(6, 1, '0+', 1 , 'system')
;

COMMIT;




