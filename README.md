docker system prune --all --volumes
docker volume rm $(docker volume ls -qf dangling=true)

docker exec -it roamdb-service psql -U myuser -d roamdb


INSERT INTO batch_execs (batch_id, batch_name) VALUES (1,'TEST');

INSERT INTO metrics_global (metric_definition_id,batch_id,date_id, value) VALUES 
(1,1,620, 12000),
(1,1,621, 12010),
(1,1,622, 12020),
(1,1,623, 12030);

INSERT INTO metrics_global (metric_definition_id,batch_id,date_id, value) VALUES 
(5,1,620, 13000),
(5,1,621, 13010),
(5,1,622, 13020),
(5,1,623, 13030);


SELECT dd.date_str, mg.value, cmt.name, crd.direction
FROM metrics_global mg 
JOIN dim_dates dd ON mg.date_id = dd.date_id
JOIN cfg_metric_definitions cmd ON mg.metric_definition_id = cmd.metric_definition_id
JOIN cfg_metric_types cmt ON cmd.metric_type_id = cmt.metric_type_id
JOIN cfg_roam_directions crd ON cmd.roam_direction_id = crd.roam_direction_id
ORDER BY mg.date_id;



SELECT dd.date_str, mg.value
FROM metrics_global mg 
JOIN dim_dates dd ON mg.date_id = dd.date_id
JOIN cfg_metric_definitions cmd ON mg.metric_definition_id = cmd.metric_definition_id
JOIN cfg_metric_types cmt ON cmd.metric_type_id = cmt.metric_type_id
JOIN cfg_roam_directions crd ON cmd.roam_direction_id = crd.roam_direction_id
WHERE crd.direction = 'IN'
AND cmt.name = 'GLOBAL'
ORDER BY mg.date_id;