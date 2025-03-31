docker exec -it roam_db psql -U myuser -d roamdb


docker cp ins_countries.sql roam_db:/tmp/ins_countries.sql
docker exec -it roam_db psql -U myuser -d roamdb -f /tmp/ins_countries.sql

docker cp ins_prefixes.sql roam_db:/tmp/ins_prefixes.sql
docker exec -it roam_db psql -U myuser -d roamdb -f /tmp/ins_prefixes.sql

docker cp ins_others.sql roam_db:/tmp/ins_others.sql
docker exec -it roam_db psql -U myuser -d roamdb -f /tmp/ins_others.sql

WITH duplicates AS (
    SELECT
        ctid,  
        ROW_NUMBER() OVER (PARTITION BY alpha2, country_name ORDER BY id) AS row_num
    FROM countries
)
DELETE FROM countries
WHERE ctid IN (
    SELECT ctid
    FROM duplicates
    WHERE row_num > 1  
);


INSERT INTO countries (alpha2, country_name)
SELECT alpha2, country_name  FROM x;



select prefix , country_alpha2, count(*) from prefixes 
group by prefix , country_alpha2 having count(*)>1;



COPY users (name, age, city)
FROM '/path/to/users.csv'
WITH (FORMAT csv, HEADER true);

COPY table_name
TO '/path/to/file.csv'
WITH (FORMAT CSV, HEADER);


docker exec -it roam_db psql -U myuser -d roamdb -c "\COPY countries TO '/home/countries.csv' WITH (FORMAT CSV, HEADER);"
docker cp roam_db:/home/countries.csv ./countries.csv

COPY countries (country_alpha2, country_name)
FROM '/tmp/countries.csv'
WITH (FORMAT csv, HEADER true);
