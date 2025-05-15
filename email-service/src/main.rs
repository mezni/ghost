use axum::{Json, Router, routing::post};
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::Mailbox,
    transport::smtp::authentication::Credentials,
};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Deserialize)]
struct EmailRequest {
    to: String,
    subject: String,
    body: String,
}

async fn send_email(Json(req): Json<EmailRequest>) -> &'static str {
    // Build the email message
    let email = Message::builder()
        .from("noreply@gmail.com".parse::<Mailbox>().unwrap())
        .to(req.to.parse::<Mailbox>().unwrap())
        .subject(req.subject)
        .body(req.body)
        .unwrap();

    // Set up credentials (use an App Password for Gmail)
    let creds = Credentials::new(
        "noreply@gmail.com".to_string(), // Your Gmail address
        "your_app_password".to_string(), // Your Gmail App Password
    );

    // Build the mailer with authentication
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email and handle the result
    match mailer.send(email).await {
        Ok(_) => "Email sent",
        Err(e) => {
            eprintln!("Failed to send email: {:?}", e);
            "Failed to send email"
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/send", post(send_email));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
