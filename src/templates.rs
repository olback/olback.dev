/*
 *  olback.net web server
 */

extern crate chrono;
use self::chrono::{Utc, Datelike};
use mail;

#[derive(Serialize)]
pub struct IndexTemplate {
    pub class: String,
    pub message: String,
    pub csrf: String,
    pub year: i32,
    pub mail: Option<mail::Mail>
}

impl Default for IndexTemplate {
    fn default() -> IndexTemplate {
        IndexTemplate {
            class: String::from(""),
            message: String::from(""),
            csrf: String::from(""),
            year: Utc::now().year(),
            mail: None
        }
    }
}

#[derive(Serialize)]
pub struct ErrorTemplate {
    pub code: u16,
    pub message: String
}
