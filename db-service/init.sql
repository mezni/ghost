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

CREATE TABLE IF NOT EXISTS stg_roam_out (
    batch_id INT NOT NULL,
    batch_date VARCHAR(10)  NOT NULL,      
    imsi VARCHAR(100) NOT NULL,
    msisdn VARCHAR(100) NOT NULL,
    vlr_number VARCHAR(100) NOT NULL,
    carrier_name VARCHAR(100),   
    country_name VARCHAR(100),
    country_alpha2 VARCHAR(100) 
);

CREATE INDEX idx_stg_roam_out
ON stg_roam_out (batch_id);


CREATE TABLE IF NOT EXISTS dim_carriers (
    id SERIAL PRIMARY KEY,  
    country_name VARCHAR(100),
    carrier_name VARCHAR(100), 
    country_alpha2 VARCHAR(100)     
);

CREATE TABLE IF NOT EXISTS dim_imsi (
    id SERIAL PRIMARY KEY,  
    imsi VARCHAR(100) NOT NULL    
);

CREATE INDEX idx_dim_imsi
ON dim_imsi (imsi);

CREATE TABLE IF NOT EXISTS dim_msisdn (
    id SERIAL PRIMARY KEY,  
    msisdn VARCHAR(100) NOT NULL    
);

CREATE INDEX idx_dim_msisdn
ON dim_msisdn (msisdn);

CREATE TABLE dim_time (
    id SERIAL PRIMARY KEY,
    date DATE NOT NULL,
    year INT NOT NULL,
    quarter INT NOT NULL,
    month INT NOT NULL,
    day INT NOT NULL,
    day_of_week INT NOT NULL,
    day_name VARCHAR(10) NOT NULL,
    week_of_year INT NOT NULL,
    is_weekend BOOLEAN NOT NULL,
    date_text VARCHAR(10) -- Text format YYYY-MM-DD
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
FROM GENERATE_SERIES('2025-01-01'::DATE, '2025-12-31'::DATE, '1 day'::INTERVAL) AS d;


CREATE TABLE IF NOT EXISTS fct_roam_out (
    batch_id INT NOT NULL,
    date_id INT NOT NULL,      
    imsi_id INT NOT NULL,
    msisdn_id INT NOT NULL,
    carrier_id INT NOT NULL
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