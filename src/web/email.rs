use anyhow::Context;
use axum::body::Bytes;
use axum::extract::Query;
use axum::http;
use lettre::message::header::ContentType;
use lettre::message::{Attachment, Mailbox, MultiPart, SinglePart};
use lettre::transport::smtp::{authentication::Credentials, AsyncSmtpTransport};
use lettre::{AsyncTransport, Message, Tokio1Executor};
use serde::{Deserialize, Serialize};

// use super::models;

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailData {
    pub school: String,
    pub contact_person: String,
    pub contact_number: String,
}

pub async fn send_email(
    Query(data): Query<EmailData>,
    image: Bytes,
) -> Result<http::StatusCode, http::StatusCode> {
    println!("Sending receipt...");

    let address = std::env::var("EMAIL_ADDRESS").expect("EMAIL_ADDRESS env not found.");
    let password = std::env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD env not found.");
    let school = data.school;
    let contact_person = data.contact_person;
    let contact_number = data.contact_number;

    let sender: Mailbox = format!("Sender <{address}>")
        .parse()
        .expect("Failed to parse sender.");

    // TODO: Make sure to hide receiver's address
    // Change it to the official receiver
    let receiver: Mailbox = format!("Receiver <{address}>")
        .parse()
        .expect("Failed to parse receiver.");

    let creds = Credentials::new(address, password);
    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
            .expect("SMTP relay error.")
            .credentials(creds)
            .build();

    let content_type = "image/jpeg";

    let receipt = Attachment::new("receipt.jpg".to_string()).body(
        image.to_vec(),
        ContentType::parse(content_type)
            .context("Failed to parse content type.")
            .unwrap(),
    );

    let email_html = format!(
        r#"
        <p><strong>School:</strong> {school}</p>
        <p><strong>Contact Person:</strong> {contact_person}</p>
        <p><strong>Contact Number:</strong> {contact_number}</p>
    "#
    );

    let create_message = Message::builder()
        .from(sender)
        .to(receiver)
        .subject("GCash Payment Receipt")
        // .header(ContentType::TEXT_PLAIN)
        // .body("Hello, World".to_string())
        .multipart(
            MultiPart::mixed()
                .singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_HTML)
                        .body(email_html),
                )
                .singlepart(receipt),
        );

    match create_message {
        Ok(email) => {
            mailer.send(email).await.expect("Failed to send email.");

            println!("Email sent successfully!");

            Ok(http::StatusCode::OK)
        }
        Err(err) => {
            eprintln!("Failed to create message: {err:?}");
            Err(http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
