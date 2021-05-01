use lettre::{smtp::authentication::IntoCredentials, EmailAddress, Envelope};
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;

use crate::config::CONFIG;

/// This function is used to send a mail with the given subject and body.
///
/// It always sends the body as html
/// All SMTP Data is read from the config file.
/// From header is created with the config domain like: zorius@`domain`
// TODO:    1. return the result to report if the emails failed to send
//          2. remove `unwraps` and `expects` and replace them
pub fn mailer(subject: &str, body: &str) {
    if !CONFIG.mailer.enable_mailer {
        return;
    }

    let smtp_address: &str = CONFIG.mailer.smtp_address.as_ref();
    let username: &str = CONFIG.mailer.smtp_username.as_ref();
    let password: &str = CONFIG.mailer.smtp_password.as_ref();

    let from = EmailAddress::new(format!("zorius@{}", CONFIG.domain)).unwrap();
    let to = EmailAddress::new(CONFIG.mailer.email_send_to.clone()).unwrap();
    let envelope = Envelope::new(Some(from), vec![to]).unwrap();

    let email = EmailBuilder::new()
        .envelope(envelope)
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
