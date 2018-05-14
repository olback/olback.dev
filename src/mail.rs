/*
 *  olback.net web server
 */

use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::{EmailTransport, SmtpTransport};
use lettre::smtp::extension::ClientId;
use lettre::smtp::ConnectionReuseParameters;
use lettre_email::EmailBuilder;

use conf::{HOST, USER, PASS, FROM, NAME, SITE};

#[derive(FromForm)]
pub struct Mail {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub body: String,
    pub copy: bool
}

pub fn send(mail_data: Mail) -> bool {

    let email;

    if mail_data.copy {
        email = EmailBuilder::new()
        .to((FROM.to_string(), NAME.to_string()))
        .cc((format!("{}", &mail_data.email), format!("{}", &mail_data.name)))
        .from((FROM.to_string(), SITE.to_string()))
        .subject(mail_data.subject)
        .text(format!("Name: {}\nEmail: {}\n\n{}", &mail_data.name, &mail_data.email, &mail_data.body))
        .build()
        .unwrap();
    } else {
        email = EmailBuilder::new()
        .to((FROM.to_string(), NAME.to_string()))
        // .cc((format!("{}", &mail_data.email), format!("{}", &mail_data.name)))
        .from((FROM.to_string(), SITE.to_string()))
        .subject(mail_data.subject)
        .text(format!("Name: {}\nEmail: {}\n\n{}", &mail_data.name, &mail_data.email, &mail_data.body))
        .build()
        .unwrap();
    }
    
    // Connect to a remote server on a custom port
    let mut mailer = SmtpTransport::simple_builder(HOST).unwrap()
    .hello_name(ClientId::Domain(HOST.to_string()))
    .credentials(Credentials::new(USER.to_string(), PASS.to_string()))
    .smtp_utf8(true)
    .authentication_mechanism(Mechanism::Login)
    .connection_reuse(ConnectionReuseParameters::ReuseUnlimited).build();
    
    let result = mailer.send(&email);

    mailer.close();

    return result.is_ok()

}