/countries
    GET: list last 50 countries
    POST: create a new country


db connection:
- docker exec -it database psql -U myuser -d roamdb



curl -X POST http://localhost:8080/countries \
     -H "Content-Type: application/json" \
     -d '{"name": "France", "iso": "FR"}'

curl -X DELETE http://localhost:8080/countries/252





Country,Operator,Ownership,PLMN,MCC,MNC,TADIG,ISO,tech_2g,tech_3g,tech_lte


CREATE TABLE networks_work (
    country TEXT,    
    operator TEXT, 
    ownership TEXT, 
    plmn TEXT, 
    mcc TEXT,                 
    mnc  TEXT, 
    tadig TEXT, 
    iso TEXT,  
    tech_2g TEXT,  
    tech_3g TEXT,  
    tech_lte TEXT        
);


\COPY networks_work FROM '/tmp/networks.csv' WITH (FORMAT csv, HEADER true);


response = client.get_cost_and_usage(
    TimePeriod={
        'Start': 'string',
        'End': 'string'
    },
    Granularity='DAILY'|'MONTHLY'|'HOURLY',
    Filter={
        'Or': [
            {'... recursive ...'},
        ],
        'And': [
            {'... recursive ...'},
        ],
        'Not': {'... recursive ...'},
        'Dimensions': {
            'Key': 'AZ'|'INSTANCE_TYPE'|'LINKED_ACCOUNT'|'LINKED_ACCOUNT_NAME'|'OPERATION'|'PURCHASE_TYPE'|'REGION'|'SERVICE'|'SERVICE_CODE'|'USAGE_TYPE'|'USAGE_TYPE_GROUP'|'RECORD_TYPE'|'OPERATING_SYSTEM'|'TENANCY'|'SCOPE'|'PLATFORM'|'SUBSCRIPTION_ID'|'LEGAL_ENTITY_NAME'|'DEPLOYMENT_OPTION'|'DATABASE_ENGINE'|'CACHE_ENGINE'|'INSTANCE_TYPE_FAMILY'|'BILLING_ENTITY'|'RESERVATION_ID'|'RESOURCE_ID'|'RIGHTSIZING_TYPE'|'SAVINGS_PLANS_TYPE'|'SAVINGS_PLAN_ARN'|'PAYMENT_OPTION'|'AGREEMENT_END_DATE_TIME_AFTER'|'AGREEMENT_END_DATE_TIME_BEFORE'|'INVOICING_ENTITY'|'ANOMALY_TOTAL_IMPACT_ABSOLUTE'|'ANOMALY_TOTAL_IMPACT_PERCENTAGE',
            'Values': [
                'string',
            ],
            'MatchOptions': [
                'EQUALS'|'ABSENT'|'STARTS_WITH'|'ENDS_WITH'|'CONTAINS'|'CASE_SENSITIVE'|'CASE_INSENSITIVE'|'GREATER_THAN_OR_EQUAL',
            ]
        },
        'Tags': {
            'Key': 'string',
            'Values': [
                'string',
            ],
            'MatchOptions': [
                'EQUALS'|'ABSENT'|'STARTS_WITH'|'ENDS_WITH'|'CONTAINS'|'CASE_SENSITIVE'|'CASE_INSENSITIVE'|'GREATER_THAN_OR_EQUAL',
            ]
        },
        'CostCategories': {
            'Key': 'string',
            'Values': [
                'string',
            ],
            'MatchOptions': [
                'EQUALS'|'ABSENT'|'STARTS_WITH'|'ENDS_WITH'|'CONTAINS'|'CASE_SENSITIVE'|'CASE_INSENSITIVE'|'GREATER_THAN_OR_EQUAL',
            ]
        }
    },
    Metrics=[
        'string',
    ],
    GroupBy=[
        {
            'Type': 'DIMENSION'|'TAG'|'COST_CATEGORY',
            'Key': 'string'
        },
    ],
    BillingViewArn='string',
    NextPageToken='string'
)

metrics
roam in count by date
roam in count by date by country
roam in count by date by country by operator

perf:
roam out by date by country by operator percentage 