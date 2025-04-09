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

