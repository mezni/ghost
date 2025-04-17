docker exec -it roam_db psql -U myuser -d roamdb

cleanup 
docker compose stop loader-service
docker compose rm -f loader-service
docker rmi alpha/loader-service:latest
docker compose build loader-service
docker compose up -d loader-service

http-server -p 8080 --cors

NOTES 
remove .env under source



INSERT INTO sor_plan (country_id,operator_id,rate, routage)
VALUES (80,9,15,NULL); -- free
INSERT INTO sor_plan (country_id,operator_id,rate, routage)
VALUES (80,10,0,NULL); -- Bouygues
INSERT INTO sor_plan (country_id,operator_id,rate, routage)
VALUES (80,11,0,NULL);  -- SFR
INSERT INTO sor_plan (country_id,operator_id,rate, routage)
VALUES (80,12,85,NULL);  -- Orange



INSERT INTO rules (name , description, is_active) VALUES ('imsi_is_not_local','IMSI non local',TRUE);
INSERT INTO rules (name , description, is_active) VALUES ('local_vlr_number','vlr_number Local ',TRUE);
INSERT INTO rules (name , description, is_active) VALUES ('sor_plan_bar','Barring operator',TRUE);
INSERT INTO rules (name , description, is_active) VALUES ('sor_plan_deviation','Deviation SoR',TRUE);


INSERT INTO notifications (date_id, batch_id,rule_id,ref_id,message) 
SELECT date_id, batch_id, rule_id, ref_id , 'operateur: '||operator||' config: '||rate||' reel: '||percent FROM (
SELECT 
    agg.date_id, 
    agg.batch_id, 
    (SELECT id FROM rules WHERE name = 'imsi_is_not_local') AS rule_id, 
    999999999 AS ref_id,
    pln.rate,
    agg.percent,
    ope.operator
FROM 
(
    SELECT *
    FROM fct_sor_out fct
    WHERE batch_id = 37
    AND country_id IN (SELECT country_id FROM sor_plan)
) agg
LEFT JOIN sor_plan pln 
    ON agg.country_id = pln.country_id 
    JOIN dim_operators ope ON pln.operator_id = ope.id
    AND agg.operator_id = pln.operator_id
WHERE agg.percent NOT BETWEEN COALESCE(pln.rate::float, 0) - 2 
                   AND COALESCE(pln.rate::float, 0) + 2);




update dim_imsi set imsi = '705010143347282' where id =1 ;

INSERT INTO notifications (date_id, batch_id,rule_id,ref_id,message)
SELECT 
    fct.date_id, 
    fct.batch_id, 
    (SELECT id FROM rules WHERE name = 'imsi_is_not_local') AS rule_id,
    ims.id, 
    ims.imsi
FROM dim_imsi ims 
JOIN fct_roam_out fct ON fct.imsi_id = ims.id
JOIN dim_roam_type typ ON ims.roam_type_id = typ.id
WHERE typ.roam_type = 'OUT'
AND ims.imsi IS NOT NULL
AND fct.batch_id = 1
AND ims.imsi NOT LIKE '60501%';








 date_id | batch_id | country_id | operator_id | imsi_id | msisdn_id | vlr_number_id 
---------+----------+------------+-------------+---------+-----------+---------------

SELECT *
FROM fct_roam_out fct JOIN dim_vlr_number vlr ON fct.vlr_number_id = vlr.id
WHERE 1=1
AND fct.batch_id = 1
AND vlr_number like '216%';



SELECT *
FROM fct_roam_out fct JOIN dim_vlr_number vlr ON fct.vlr_number_id = vlr.id
WHERE 1=1
AND fct.batch_id = 1
AND vlr_number like '216%';

SELECT *
FROM fct_roam_out fct JOIN dim_vlr_number vlr ON fct.vlr_number_id = vlr.id
WHERE 1=1
AND fct.batch_id = 1
AND vlr_number like '60501%';


SELECT distinct ( pre.prefix)
FROM dim_msisdn msi 
JOIN fct_roam_out fct ON fct.msisdn_id = msi.id
JOIN dim_prefixes pre ON msi.msisdn LIKE pre.prefix || '%'
WHERE fct.batch_id = 1;

select * from dim_roam_type where roam_type = 'OUT'

WITH (SELECT distinct ( pre.prefix) as prefix
FROM dim_msisdn msi 
JOIN fct_roam_out fct ON fct.msisdn_id = msi.id
JOIN dim_prefixes pre ON msi.msisdn LIKE pre.prefix || '%'
WHERE fct.batch_id = 1) as prefix
select vlr_number
from dim_vlr_number vlr 
JOIN dim_roam_type typ ON vlr.roam_type_id = typ.id
JOIN fct_roam_out fct ON fct.vlr_number_id = vlr.id
WHERE typ.roam_type = 'OUT'
AND vlr_number like prefix.prefix|| '%'
AND fct.batch_id = 1;




WITH prefixes AS (
    SELECT DISTINCT pre.prefix as prefix
    FROM dim_msisdn msi 
    JOIN fct_roam_out fct ON fct.msisdn_id = msi.id
    JOIN dim_prefixes pre ON msi.msisdn LIKE pre.prefix || '%'
    WHERE fct.batch_id = 1
)
SELECT DISTINCT vlr.vlr_number
FROM dim_vlr_number vlr 
JOIN dim_roam_type typ ON vlr.roam_type_id = typ.id
JOIN fct_roam_out fct ON fct.vlr_number_id = vlr.id
JOIN prefixes p ON vlr.vlr_number LIKE p.prefix || '%'
WHERE typ.roam_type = 'OUT'
AND fct.batch_id = 1;
