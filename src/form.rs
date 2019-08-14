/*
 *  olback.net web server
 */

extern crate mailchecker;

pub trait Validate {
    fn validate(&self) -> (bool, Vec<String>);
}

#[derive(FromForm, Clone, Serialize, Deserialize)]
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
    fn validate(&self) -> (bool, Vec<String>) {

        let mut v: Vec<String> = Vec::new();

        if self.name.is_empty() {
            v.push(String::from("Name is empty"));
        }

        if !mailchecker::is_valid(self.email.as_str()) {
            v.push(String::from("Email is invalid"));
        }

        if self.subject.is_empty() {
            v.push(String::from("Subject is empty"));
        }

        if self.body.is_empty() {
            v.push(String::from("Email body is empty"));
        }

        if v.len() == 0 {
            return (true, v)
        }

        (false, v)
    }
}

