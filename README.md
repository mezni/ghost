POST /api/events {"intervalStart":"", "intervalDuration":"", "TrxCount":"" } -- default now, 1min, 10000
GET /api/health
GET /api/metrics

implement logic
add health
add logs
add metrics

1- kafkaLoader
2- FastProcess (pyspark/cassandra)
3- BatchProcess (pyspark/cassandra)
4- DWH
5- Visu
6- deploy k8s/aws/gcp/azure


