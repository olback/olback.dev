/*
 *  olback.net web server
 */

extern crate chrono;
use self::chrono::{Utc, Datelike};
use form;
use birthday;

#[derive(Serialize)]
pub struct IndexTemplate {
    pub class: String,
    pub message: String,
    pub csrf: String,
    pub year: i32,
    pub age: u8,
    pub mail: Option<form::Mail>
}

impl Default for IndexTemplate {
    fn default() -> IndexTemplate {
        IndexTemplate {
            class: String::from(""),
            message: String::from(""),
            csrf: String::from(""),
            year: Utc::now().year(),
            age: birthday::get_age(),
            mail: None
        }
    }
}

#[derive(Serialize)]
pub struct ErrorTemplate {
    pub code: u16,
    pub message: String
}
