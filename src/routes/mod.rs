/*
 *  olback.net web server
 */

// use rocket::request::{FlashMessage};
// use rocket::http::{Cookie, Cookies};
use rocket::{
    get,
    request::FlashMessage,
    http::{Cookie, Cookies}
};
use rocket_contrib::templates::Template;
use csrf::{AesGcmCsrfProtection, CsrfProtection};
use super::conf;
use super::templates;

// Re exports
pub mod files;
pub mod redirects;
pub mod mail;
pub mod utils;

#[get("/")]
pub fn index(flash: Option<FlashMessage>, mut cookies: Cookies) -> Template {

    let protect = AesGcmCsrfProtection::from_key(conf::get_aes_key());

    // Generate token/cookie pair
    let (csrf_token, csrf_cookie) = protect.generate_token_pair(None, 18000)
        .expect("couldn't generate token/cookie pair");

    // Set csrf cookie & drop cookies
    cookies.add_private(Cookie::new(".csrf", csrf_cookie.b64_string()));
    drop(cookies);

    // Construct context
    let mut context = templates::IndexTemplate {
        csrf: csrf_token.b64_string(),
        ..Default::default()
    };

    // If it's a flash response, set class and message
    if flash.is_some() {
        flash.map(|f| {
            context.class = f.name().to_string();
            context.message = f.msg().to_string();
        }).unwrap();
    }

    return Template::render("index", &context)
}
