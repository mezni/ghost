CREATE TABLE IF NOT EXISTS countries (
    id SERIAL PRIMARY KEY,
    country_alpha2 VARCHAR(2),
    country_name VARCHAR(100)
);

CREATE TABLE IF NOT EXISTS prefixes (
    id SERIAL PRIMARY KEY,
    prefix VARCHAR(10) NOT NULL,
    country_alpha2 VARCHAR(100),
    carrier_id VARCHAR(100),
    carrier_name VARCHAR(100),
    length VARCHAR(2)        
);


CREATE TABLE IF NOT EXISTS batch_execs (
    id SERIAL PRIMARY KEY,
    batch_name VARCHAR(100) NOT NULL,
    start_time TIMESTAMP,
    end_time TIMESTAMP,        
    batch_status VARCHAR(10) 
);


CREATE TABLE IF NOT EXISTS stg_roam_out (
    batch_id INT NOT NULL,
    batch_date VARCHAR(20)  NOT NULL,      
    imsi VARCHAR(100) NOT NULL,
    msisdn VARCHAR(100) NOT NULL,
    vlr_number VARCHAR(100) NOT NULL,
    carrier_name VARCHAR(100),   
    country_name VARCHAR(100)   
);

CREATE INDEX idx_stg_roam_out
ON stg_roam_out (batch_id);