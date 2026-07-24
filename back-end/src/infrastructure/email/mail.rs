use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{header, SinglePart};
use std::env;



fn send_mail(
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

    Ok(())
}


pub fn send_validation_email(
    destinatario_email: &str,
    destinatario_nome: &str,
    verification_url: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let subject = "Validação de email";

    let body = format!(
        r#"
            <html>
            <body style="font-family: sans-serif; text-align: center;">
                <h2>Bem vindo ao Prateleira!</h2>
                <p>Para concluir seu cadastro, favor clicar no botão para validação do email</p>
                <a href="{}" style="
                    display: inline-block;
                    padding: 12px 24px;
                    background-color: #4CAF50;
                    color: white;
                    text-decoration: none;
                    border-radius: 4px;
                    font-weight: bold;
                ">
                    Clique aqui
                </a>
            </body>
            </html>
        "#,
        verification_url
    );

    send_mail(destinatario_email, destinatario_nome, subject, body)
}

pub fn send_identity_verification(
    destinatario_email: &str,
    destinatario_nome: &str,
    verification_url: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let subject = "Conferência de identidade";

    let body = format!(
        r#"
            <html>
            <body style="font-family: sans-serif; text-align: center;">
                <h2>Conferência de identidade</h2>
                <p>Houve uma tentativa de login em uma nova localização, caso seja você favor clicar no botão</p>
                <a href="{}" style="
                    display: inline-block;
                    padding: 12px 24px;
                    background-color: #4CAF50;
                    color: white;
                    text-decoration: none;
                    border-radius: 4px;
                    font-weight: bold;
                ">
                    Clique aqui
                </a>
            </body>
            </html>
        "#,
        verification_url
    );

    send_mail(destinatario_email, destinatario_nome, subject, body)
}