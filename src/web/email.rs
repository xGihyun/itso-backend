use axum::body::Bytes;
use axum::extract::State;
use axum::http;
use lettre::message::header::{ContentDisposition, ContentType};
use lettre::message::SinglePart;
use lettre::transport::smtp::{authentication::Credentials, AsyncSmtpTransport};
use lettre::{AsyncTransport, Message, Tokio1Executor};

use super::models;

pub async fn send_email(
    State(email): State<models::EmailCredentials>,
    image: Bytes,
) -> Result<http::StatusCode, http::StatusCode> {
    println!("Sending receipt...");

    let address = email.address.to_owned();
    let password = email.password.to_owned();

    let sender = format!("Sender <{address}>")
        .parse()
        .expect("Failed to parse sender.");

    // TODO: Make sure to hide receiver's address
    // Change it to the official receiver
    let receiver = format!("Receiver <{address}>")
        .parse()
        .expect("Failed to parse receiver.");

    let creds = Credentials::new(address, password);
    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
            .expect("SMTP relay error.")
            .credentials(creds)
            .build();

    let create_message = Message::builder()
        .from(sender)
        .to(receiver)
        .subject("GCash Payment Receipt")
        .singlepart(
            SinglePart::builder()
                .header(ContentType::parse("image/jpeg").unwrap())
                .header(ContentDisposition::attachment("receipt.jpg"))
                .body(image.to_vec()),
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
