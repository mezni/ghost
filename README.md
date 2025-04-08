

[workspace]
members = [
    "core",
    "loader-service",
]


cargo new --lib core
cargo new --bin loader-service


cargo run --bin loader-service --target-dir bin

cargo run -p loader-service


select country_name, carrier_name , count(*) 
from stg_roam_out
where country_name = 'France'
group by country_name, carrier_name 
;



country_name | carrier_name | count 
--------------+--------------+-------
 France       | Orange       | 10384
 France       | Free         |  1604
 France       | Bouygues     |    38
 France       | SFR          | 15197



roamdb=# select  count(*) 
from stg_roam_out
where country_name = 'France' 
;
 count 
-------
 27223



roamdb=# select country_name, carrier_name , count(*) 
from stg_roam_out
where country_name = 'France' 
group by country_name, carrier_name 
;
 country_name | carrier_name | count 
--------------+--------------+-------
 France       | Orange       | 10384
 France       | Free         |  1604
 France       | Bouygues     |    38
 France       | SFR          | 15197
(4 rows)



INSERT INTO streeing_config (operator_id, rate) VALUES (1,'85'); -- orange
INSERT INTO streeing_config (operator_id, rate) VALUES (2,''); -- SFR
INSERT INTO streeing_config (operator_id, rate) VALUES (3,''); -- Bouygues
INSERT INTO streeing_config (operator_id, rate) VALUES (5,'15'); -- Free




DELETE FROM prefixes
WHERE prefix IN (
    SELECT prefix FROM (
        SELECT prefix,
               ROW_NUMBER() OVER (PARTITION BY prefix ORDER BY id) AS rn
        FROM prefixes
    ) t
    WHERE t.rn > 1
);
