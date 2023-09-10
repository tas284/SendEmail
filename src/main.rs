use std::io::stdin;
use std::env;
use lettre::{
    message::{header, MultiPart, SinglePart},
    SmtpTransport, Message, Transport, transport::smtp::authentication::Credentials
};

fn main() {

    let mut email_to = String::new();
    let mut email_subject = String::new();
    let mut email_body = String::new();

    email_to = read_data("Informe o email do destinatario: ", email_to);
    email_subject = read_data("Informe o titulo: ", email_subject);
    email_body = read_data("Informe o conteudo do email: ", email_body);

    let username = get_env("USERNAME_EMAIL_RUST");
    let password = get_env("PASSWORD_EMAIL_RUST");
    let smtp = get_env("SMTP_EMAIL_RUST");

    let credentials = Credentials::new(username, password);

    let html = build_html(&email_subject, &email_body);

    let email = Message::builder()
        .from("Tiago Saldanha <tiago.sigeatende@gmail.com>".parse().unwrap())
        .to(format!("<{}>", email_to).parse().unwrap())
        .subject("Send Email V2")
        .multipart(
            MultiPart::alternative()
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(String::from("Hello from Lettre! A mailer library for Rust")),
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(html),
                ),
        )
        .expect("failed to build email");

    let mailer = SmtpTransport::relay(&smtp)
        .unwrap()
        .credentials(credentials)
        .build();
    
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}

fn read_data(message: &str, mut my_string: String) -> String {
    println!("{}", message);

    let stdin = stdin();
    stdin.read_line(&mut my_string)
        .expect("Falha ao ler dados do usuario!");

    my_string.trim().to_string()

}

fn get_env(variable: &str) -> String {
    match env::var(variable) {
        Ok(value) => value,
        Err(_) => "variable environment not set".to_string(),
    }
}

fn build_html(title: &str, content: &str) -> String {
    let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
</head>
<body>
    <div style="display: flex; flex-direction: column; align-items: left;">
        <div style="font-family: Arial, Helvetica, sans-serif;">{}</div>
    </div>
</body>
</html>"#, title, content);

    html
}