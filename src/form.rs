/*
 *  olback.net web server
 */

extern crate mailchecker;
use rocket::http::Cookie;
use data_encoding::BASE64;

pub trait Validate {
    fn validate(&self) -> bool;
}

pub trait Refill {
    fn refill(&self) -> (bool, Cookie<'static>);
}

#[derive(FromForm, Serialize, Deserialize)]
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

impl Refill for Mail {
    fn refill(&self) -> (bool, Cookie<'static>) {
        let serialized = match bincode::serialize(self) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                return (false, Cookie::new(".form-data", ""))
            }
        };

        let encoded = BASE64.encode(&serialized);
        let cookie = Cookie::new(".form-data", encoded);

        (true, cookie)
    }
}
