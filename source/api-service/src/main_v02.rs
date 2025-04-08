use actix_web::{App, HttpResponse, HttpServer, web};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use tokio_postgres::{Error, NoTls};

#[derive(Serialize, Deserialize)]
struct Kpi {
    key: String,
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct KpiResponse {
    kpis: Vec<Kpi>,
}

async fn get_kpis(client: web::Data<Arc<tokio_postgres::Client>>) -> HttpResponse {
    let rows = client
        .query(
            "select country_name, count(*) from stg_roam_out group by country_name",
            &[],
        )
        .await
        .unwrap();

    let mut kpis = Vec::new();
    for row in rows {
        let country_name: String = row.get(0);
        let count: i64 = row.get(1);

        kpis.push(Kpi {
            key: country_name,
            value: count.to_string(),
            date: None,
        });
    }

    let response = KpiResponse { kpis };

    HttpResponse::Ok().json(response)
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
            .route("/kpis", web::get().to(get_kpis))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}