/*
 *  olback.net web server
 */

use mail::Mail;

#[derive(Serialize)]
pub struct IndexTC {
    pub class: String,
    pub message: String
}

#[derive(Serialize)]
pub struct ErrorTemplate {
    pub code: i16,
    pub message: String
}

pub fn check_form_data(mail_data: &Mail) -> bool {

    if mail_data.name.is_empty() {
        // return format!("Name may not be empty");
        return false;
    }

    if mail_data.email.is_empty() {
        // return format!("Email may not be empty");
        return false;
    }

    if mail_data.subject.is_empty() {
        // return format!("Subject may not be empty");
        return false;
    }

    if mail_data.body.is_empty() {
        // return format!("Mail body may not be empty");
        return false;
    }

    true

}
