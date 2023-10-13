use axum::body::Bytes;
// use axum::response::{IntoResponse, Result};
use lettre::message::header::{ContentDisposition, ContentType};
use lettre::message::SinglePart;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use reqwest::StatusCode;

pub async fn send_email(image: Bytes) -> StatusCode {
    println!("Sending receipt...");

    let sender = "IT Olympics <giordnnuz27@gmail.com>"
        .parse()
        .expect("Failed to parse sender.");
    let receiver = "Gihyun <giordnnuz@gmail.com>"
        .parse()
        .expect("Failed to parse receiver.");

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
            let creds = Credentials::new(
                "giordnnuz27@gmail.com".to_owned(),
                "myqu jeyf unsv zysg".to_owned(),
            );

            let mailer = SmtpTransport::relay("smtp.gmail.com")
                .unwrap()
                .credentials(creds)
                .build();

            let send_email = mailer.send(&email);

            match send_email {
                Ok(_) => {
                    println!("Email sent successfully!");
                }
                Err(err) => {
                    eprintln!("Failed to send email: {err:?}");
                }
            }
        }
        Err(err) => eprintln!("Failed to create message: {err:?}"),
    }

    StatusCode::OK
}
