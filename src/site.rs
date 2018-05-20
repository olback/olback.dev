/*
 *  olback.net web server
 */

use mail::Mail;
use mailchecker;

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

    if !mailchecker::is_valid(mail_data.email.as_str()) {
        return false;
    }

    if mail_data.name.is_empty() {
        return false;
    }

    if mail_data.subject.is_empty() {
        return false;
    }

    if mail_data.body.is_empty() {
        return false;
    }

    true

}
