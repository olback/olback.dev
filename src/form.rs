/*
 *  olback.net web server
 */

extern crate mailchecker;

pub trait Validate {
    fn validate(&self) -> bool;
}

#[derive(FromForm, Serialize)]
pub struct Mail {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub body: String,
    pub copy: bool,
    pub _csrf: String,
    pub _interactive: bool,
}

impl Validate for Mail {
    fn validate(&self) -> bool {
        if !mailchecker::is_valid(self.email.as_str()) {
            return false;
        }

        if self.name.is_empty() {
            return false;
        }

        if self.subject.is_empty() {
            return false;
        }

        if self.body.is_empty() {
            return false;
        }

        true
    }
}
