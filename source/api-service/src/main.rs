use actix_web::{App, HttpResponse, HttpServer, web};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use tokio_postgres::{Error, NoTls};

#[derive(Serialize, Deserialize)]
struct Record {
    key: String,
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct SummaryResponse {
    data: Vec<Record>,
}

async fn get_summary(client: web::Data<Arc<tokio_postgres::Client>>) -> HttpResponse {
    let mut data = Vec::new();

    let row = client
        .query_one("SELECT COUNT(*) FROM fct_roam_out", &[])
        .await
        .unwrap();

    let roam_out_count: i64 = row.get(0);

    data.push(Record {
        key: "Roamer Out".to_string(),
        value: roam_out_count.to_string(),
        date: None,
    });

    data.push(Record {
        key: "Roamer In".to_string(),
        value: "10233".to_string(),
        date: None,
    });

    data.push(Record {
        key: "Anomalies".to_string(),
        value: "23".to_string(),
        date: None,
    });

    data.push(Record {
        key: "Notification".to_string(),
        value: "5".to_string(),
        date: None,
    });

    let summary = SummaryResponse { data };

    HttpResponse::Ok().json(summary)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let db = env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");

    let database_url = format!(
        "host=localhost user={} password={} dbname={}",
        user, password, db
    );
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .expect("Failed to connect to database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let client = Arc::new(client);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .route("/analytics/summary", web::get().to(get_summary))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
