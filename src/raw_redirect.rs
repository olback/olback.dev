/*
 *  olback.net web server
 */

use rocket::{
    response::Responder,
    http::hyper::header::Location
};

// https://github.com/SergioBenitez/Rocket/issues/842
#[derive(Responder)]
#[response(status=303)]
pub struct RawRedirect(pub (), pub Location);
