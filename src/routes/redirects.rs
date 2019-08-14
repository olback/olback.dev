/*
 *  olback.net web server
 */

use raw_redirect::RawRedirect;
use rocket::http::hyper::header::Location;

#[get("/contact")]
pub fn contact_to_contact() -> RawRedirect {
    // Redirect::to("/#contact")
    RawRedirect((), Location(String::from("/#contact")))
}

#[get("/mail")]
pub fn mail_to_contact() -> RawRedirect {
    contact_to_contact()
}
