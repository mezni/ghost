- API
POST /api/analytics

{
  "metrics": [
    { "type": "sum", "field": "revenue" },
    { "type": "avg", "field": "session_duration" },
    { "type": "count", "field": "*" }
  ],
  "dimensions": ["country", "platform"],
  "filters": {
    "country": ["US", "CA"],
    "platform": ["iOS"],
    "date": { "from": "2024-01-01", "to": "2024-12-31" }
  }
}

{
  "data": [
    {
      "country": "US",
      "platform": "iOS",
      "sum_revenue": 123456.78,
      "avg_session_duration": 305.6,
      "count": 8923
    },
    {
      "country": "CA",
      "platform": "iOS",
      "sum_revenue": 45678.90,
      "avg_session_duration": 276.1,
      "count": 3123
    }
  ]
}


docker exec -it roam_db psql -U myuser -d roamdb

-- cargo new backend

cargo new core --lib
cargo new loader-service
cargo new analytics-service
cargo new api-service

[workspace]
members = [
    "core",
    "loader-service",
    "analytics-service",
    "api-service",
]

cargo build --bin loader-service --target-dir bin
cargo run -p loader-service


- DB



CREATE TABLE policies (
    policy_id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    policy_type VARCHAR(50) NOT NULL, -- 'scaling', 'security', 'compliance', etc.
    version VARCHAR(20) NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(255) NOT NULL,
    updated_by VARCHAR(255),
    metadata JSONB
);


CREATE TABLE policy_rules (
    rule_id VARCHAR(36) PRIMARY KEY,
    policy_id VARCHAR(36) REFERENCES policies(policy_id),
    rule_name VARCHAR(255) NOT NULL,
    rule_condition TEXT NOT NULL, -- Could be JSON or condition DSL
    rule_action TEXT NOT NULL, -- Could be JSON or action DSL
    priority INTEGER NOT NULL,
    is_enabled BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);




SELECT msisdn 
FROM fct_roam_out fct
JOIN dim_msisdn msi on fct.msisdn_id = msi.id


date_id | batch_id | country_id | operator_id | imsi_id | msisdn_id | vlr_number_id

select * 
from fct_roam_out ;


    {
      "country": "US",
      "platform": "iOS",
      "sum_revenue": 123456.78,
      "avg_session_duration": 305.6,
      "count": 8923
    },




Purpose	Current Name	Suggested RESTful Name	Notes
Health check	/health	/health	👍 Already good – short and standard.
Overview stats summary	/overview	/stats/overview	Makes it clear it’s a summary/statistical overview.
Roam out counts by date	/roam-out-counts	/roaming/out/counts-by-date	Resource-oriented, organized under /roaming/out.
Test DB connection	/test	/debug/db-connection	Descriptive and clearly for debugging or testing.    


/stats/roamout-by-date
/stats/roamout-by-country
/stats/roamout-by-operator
