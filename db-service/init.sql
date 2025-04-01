-- Create table
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
    length VARCHAR(100)        
);

CREATE TABLE IF NOT EXISTS batch_execs (
    id SERIAL PRIMARY KEY,
    batch_name VARCHAR(10) NOT NULL,
    source_type VARCHAR(10),
    source_name VARCHAR(100),
    start_time TIMESTAMP,
    end_time TIMESTAMP,        
    corr_id INT,
    batch_status VARCHAR(10) 
);


-- Load CSV
COPY countries(id, country_alpha2, country_name)
FROM '/countries.csv'
DELIMITER ','
CSV HEADER;

COPY prefixes(prefix, country_alpha2, carrier_id, carrier_name,length)
FROM '/prefixes.csv'
DELIMITER ','
CSV HEADER;