/*
 *  olback.net web server
 */

use super::super::raw_redirect::RawRedirect;
use rocket::{
    get,
    http::hyper::header::Location
};

#[get("/contact")]
pub fn contact_to_contact() -> RawRedirect {
    // Redirect::to("/#contact")
    RawRedirect((), Location(String::from("/#contact")))
}

#[get("/mail")]
pub fn mail_to_contact() -> RawRedirect {
    contact_to_contact()
}
