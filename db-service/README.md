docker exec -it roam_db psql -U myuser -d roamdb


docker cp ins_countries.sql roam_db:/tmp/ins_countries.sql
docker exec -it roam_db psql -U myuser -d roamdb -f /tmp/ins_countries.sql

docker cp ins_prefixes.sql roam_db:/tmp/ins_prefixes.sql
docker exec -it roam_db psql -U myuser -d roamdb -f /tmp/ins_prefixes.sql

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