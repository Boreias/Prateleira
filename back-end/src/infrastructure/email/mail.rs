use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{header, SinglePart};
use std::env;



pub fn send_mail(
    destinatario_email: &str,
    destinatario_nome: &str,
    subject: &str,
    body: String
) -> Result<(), Box<dyn std::error::Error>> {

    let email_sender = env::var("EMAIL_SENDER").expect("EMAIL_SENDER não definido");
    let email_password = env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD não definida");

    let email = Message::builder()
        .from(format!("{} <{}>", "Prateleira", email_sender.clone()).parse()?)
        .to(format!("{} <{}>", destinatario_nome, destinatario_email).parse()?)
        .subject(subject)
        .singlepart(
            SinglePart::builder()
                .header(header::ContentType::TEXT_PLAIN)
                .body(body),
        )?;

    let creds = Credentials::new(email_sender.to_string(), email_password.to_string());

    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    mailer.send(&email)?;

    println!("✅ E-mail enviado para {}", destinatario_email.to_string());

    Ok(())
}