/*
 *  olback.net web server
 */

extern crate lettre;
extern crate lettre_email;
extern crate native_tls;

use self::lettre::{ClientSecurity, ClientTlsParameters, SmtpClient, Transport};
use self::lettre::smtp::authentication::{Credentials, Mechanism};
use self::lettre::smtp::ConnectionReuseParameters;
use self::lettre::smtp::extension::ClientId;
use self::native_tls::{Protocol, TlsConnector};
use self::lettre_email::EmailBuilder;
use conf;

#[derive(FromForm)]
pub struct Mail {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub body: String,
    pub copy: bool
}

pub fn send(mail_data: Mail) -> bool {

    let mail_config = conf::read_mail_config();
    let body: String = format!("Name: {}\nEmail: {}\n\n{}", &mail_data.name, &mail_data.email, &mail_data.body);
    let email;

    if mail_data.copy {
        email = EmailBuilder::new()
        .to((mail_config.from.clone(), mail_config.name))
        .bcc((format!("{}", &mail_data.email), format!("{}", &mail_data.name)))
        .from((mail_config.from.clone(), mail_config.site.clone()))
        .subject(mail_data.subject)
        .text(body)
        .build()
        .unwrap();
    } else {
        email = EmailBuilder::new()
        .to((mail_config.from.clone(), mail_config.name))
        // .bcc((format!("{}", &mail_data.email), format!("{}", &mail_data.name)))
        .from((mail_config.from.clone(), mail_config.site.clone()))
        .subject(mail_data.subject)
        .text(body)
        .build()
        .unwrap();
    }

    let mut tls_builder = TlsConnector::builder();
    // Disable as many security features as possible ( no luck :( )
    tls_builder.min_protocol_version(Some(Protocol::Sslv3));
    tls_builder.use_sni(false);
    tls_builder.danger_accept_invalid_certs(true);
    tls_builder.danger_accept_invalid_hostnames(true);
    let tls_parameters =
        ClientTlsParameters::new(
            "mail.olback.net".to_string(),
            tls_builder.build().unwrap()
        );

    let mut mailer = SmtpClient::new(
        ("mail.olback.net", 587), ClientSecurity::Wrapper(tls_parameters)
    ).unwrap()
        .authentication_mechanism(Mechanism::Plain) // Mechanism::Login does not work either
        .hello_name(ClientId::Domain("mail.olback.net".to_string()))
        .credentials(Credentials::new(
            "user@example.com".to_string(), "<passwd>".to_string()
        ))
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        .transport();

    let result = mailer.send(email.into());

    println!("Result: {:?}", result);

    mailer.close();

    result.is_ok() // false

}

// fn main() {

//     let sent = send(Mail {
//         name: String::from("Edwin"),
//         email: String::from("ee@olback.net"),
//         subject: String::from("Test email"),
//         body: String::from("asdf"),
//         copy: false
//     });

//     println!("Mail sent: {}", sent);

// }
