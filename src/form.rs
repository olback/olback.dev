/*
 *  olback.net web server
 */

extern crate mailchecker;
use mail::Mail;

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
