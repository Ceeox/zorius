use lettre::smtp::authentication::IntoCredentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;

use crate::config::{Settings, CONFIG};

pub fn mailer(subject: &str, body: &str) {
    let to_email = CONFIG.email_send_to.as_ref();
    let smtp_address: &str = CONFIG.smtp_address.as_ref();
    let username: &str = CONFIG.smtp_username.as_ref();
    let password: &str = CONFIG.smtp_password.as_ref();

    println!("smtp_address: {}, username: {}", smtp_address, username);
    println!("sending email to: {}\nsubject: {}", to_email, subject);

    let email = EmailBuilder::new()
        .to(to_email)
        .from(username)
        .subject(subject)
        .html(body)
        .build()
        .expect("failed to build email")
        .into();

    let credentials = (username, password).into_credentials();
    let mut client = SmtpClient::new_simple(smtp_address)
        .expect("failed to connect to smtp server")
        .credentials(credentials)
        .transport();

    let _result = client.send(email);
}
