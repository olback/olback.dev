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
    let body: String = format!("Name: {}\nEmail: {}\n\n{}",
        &mail_data.name,
        &mail_data.email,
        &mail_data.body
    );

    let mut email = EmailBuilder::new()
    .to((mail_config.to.clone(), mail_config.name))
    .from((mail_config.from.clone(), mail_config.site.clone()))
    .subject(mail_data.subject)
    .reply_to("contact@olback.net")
    .text(body);

    if mail_data.copy {
        email = email.bcc((mail_data.email, format!("{}", &mail_data.name)));
    }

    let mut tls_builder = TlsConnector::builder();
    tls_builder.min_protocol_version(Some(Protocol::Tlsv12));

    let tls_parameters = ClientTlsParameters::new(
        mail_config.host.clone(),
        tls_builder.build().unwrap()
    );

    let mut mailer = SmtpClient::new(
        (mail_config.host.clone().as_str(), mail_config.port),
        ClientSecurity::Required(tls_parameters)
    ).unwrap()
    .hello_name(ClientId::Domain(mail_config.host.clone()))
    .smtp_utf8(true)
    .credentials(Credentials::new(
        mail_config.username,
        mail_config.password
    ))
    .authentication_mechanism(Mechanism::Plain)
    .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
    .transport();

    let result = mailer.send(email.build().unwrap().into());

    // println!("Result: {:#?}", result);

    mailer.close();

    result.is_ok() // false

}
