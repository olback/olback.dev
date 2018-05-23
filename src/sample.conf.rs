use colored::*;

pub const HOST: &str = ""; // Mail server hostname
pub const USER: &str = ""; // Mail user name
pub const PASS: &str = ""; // Mail password
pub const FROM: &str = ""; // From address
pub const NAME: &str = ""; // From name
pub const SITE: &str = "olback.net"; // Root domain

pub fn check() -> bool {

    let mut pass = true;

    if HOST.is_empty() {
        println!("{}", "Field 'HOST' is empty!".bold().yellow());
        pass = false;
    }

    if USER.is_empty() {
        println!("{}", "Field 'USER' is empty!".bold().yellow());
        pass = false;
    }

    if PASS.is_empty() {
        println!("{}", "Field 'PASS' is empty!".bold().yellow());
        pass = false;
    }

    if FROM.is_empty() {
        println!("{}", "Field 'FROM' is empty!".bold().yellow());
        pass = false;
    }

    if NAME.is_empty() {
        println!("{}", "Field 'NAME' is empty!".bold().yellow());
        pass = false;
    }

    if SITE.is_empty() {
        println!("{}", "Field 'SITE' is empty!".bold().yellow());
        pass = false;
    }

    pass

}

