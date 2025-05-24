CREATE TABLE IF NOT EXISTS countries (
    country_id SERIAL PRIMARY KEY,
    iso TEXT,
    name TEXT,
    name_en TEXT,
    name_fr TEXT,
    prefix TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT,
    updated_at TIMESTAMP,
    updated_by TEXT
);


COPY countries (iso,name,name_en,name_fr,prefix)
FROM '/countries.csv'
DELIMITER ',' CSV HEADER;

UPDATE countries SET created_by = 'admin';

CREATE OR REPLACE VIEW v_prefixes AS
SELECT
  country_id,
  NULL AS operator_id,
  REPLACE(TRIM(p), '-', '') AS prefix
FROM countries,
LATERAL unnest(string_to_array(prefix, ',')) AS p
WHERE 
prefix not in ('1','590','599','262','672') OR 
iso  NOT IN ('AX','CC','CX','KZ','BV','SJ','EH','GG','IM','JE');


