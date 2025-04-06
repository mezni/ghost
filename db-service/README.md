docker exec -it roam_db psql -U myuser -d roamdb




   2 |                        | Chippie                               | CW
   3 |                        | DNA                                   | AX,FI
   4 |                        | Elisa                                 | AX,FI
   5 |                        | Maritime Communications Partner (MCP) | 001
   6 |                        | MX                                    | TelCEl
   7 |                        | Optus                                 | AU,CC,CX
   8 |                        | telenor norge                         | NO,SJ
   9 |                        | telia                                 | NO,SJ
  10 |                        | Telia                                 | AX,FI

insert into countries (country_name, country_alpha2) values ('Curaçao','CW'); 


  id  | country_name | carrier_name | country_alpha2 
-----+--------------+--------------+----------------
 203 |              |              | 
 235 | Brazil       |              | BR
 284 | Guatemala    |              | GT
 357 | Russia       |              | RU


roamdb=# select UPPER(Carrier_name), carrier_name , count(*) from dim_carriers group by UPPER(Carrier_name), carrier_name having count(*) >1;
  upper   | carrier_name | count 
----------+--------------+-------
 ORANGE   | Orange       |     4
 T-MOBILE | T-Mobile     |     4
 TELIA    | Telia        |     2
 BEELINE  | Beeline      |     2
 ETISALAT | Etisalat     |     3
 UNITEL   | Unitel       |     2
 TELENOR  | Telenor      |     2
 MOVISTAR | Movistar     |     3
 O2       | O2           |     3
 MTN      | MTN          |     2
 ELISA    | Elisa        |     2
 OOREDOO  | Ooredoo      |     2
 TELE 2   | Tele 2       |     2
 TELE2    | Tele2        |     2
 TELEKOM  | Telekom      |     2
 AIRTEL   | Airtel       |     6
 ZAIN     | Zain         |     4
 TIGO     | Tigo         |     2
          |              |     4
 A1       | A1           |     4
 TELIA    | telia        |     2
 VODAFONE | Vodafone     |     3


SELECT prefix, COUNT(*)
FROM prefixes
GROUP BY prefix
HAVING COUNT(*) > 1;



 id  | prefix  | country_alpha2 |  carrier_id   | carrier_name  | length 
------+---------+----------------+---------------+---------------+--------
 2091 | 2517001 | ET             | safaricom     | Safaricom     | 9

insert into prefixes (prefix,country_alpha2) values ('972','IL');





SELECT dc.country_name, dc.carrier_name , count(*)
FROM fct_roam_out fct, dim_carriers dc
WHERE fct.carrier_id=dc.id 
GROUP BY dc.country_name, dc.carrier_name ;


ELECT dc.country_name, dc.carrier_name , count(*)
FROM fct_roam_out fct, dim_carriers dc
WHERE fct.carrier_id=dc.id AND dc.country_name = 'Algeria'
GROUP BY dc.country_name, dc.carrier_name order by 3 desc;

                                      