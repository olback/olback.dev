/*
 *  olback.net web server
 */

use templates;
use form::{self, Validate};
use conf;
use mailer;
use rocket::response::Flash;
use rocket::request::Form;
use rocket::http::{Cookie, Cookies};
use rocket::http::hyper::header::Location;
use raw_redirect::RawRedirect;
use rocket_contrib::templates::Template;
use csrf::{AesGcmCsrfProtection, CsrfProtection};
use data_encoding::BASE64;

// TODO: Set csrf cookie
#[post("/mail", data = "<mail>")]
pub fn send_mail(mail: Form<form::Mail>, mut cookies: Cookies) -> Result<Flash<RawRedirect>, Template> {

    let mail_data = mail.into_inner();
    let protect = AesGcmCsrfProtection::from_key(conf::get_aes_key());
    let (csrf_token, new_csrf_cookie) = protect.generate_token_pair(None, 18000)
        .expect("couldn't generate token/cookie pair");

    // TODO: set csrf here
    let mut context = templates::IndexTemplate {
        csrf: csrf_token.b64_string(),
        class: String::from("error"),
        mail: Some(mail_data.clone()),
        ..Default::default()
    };

    /*
    *  csrf check
    */
    let csrf_cookie = match cookies.get_private(".csrf") {
        Some(v) => v.value().to_string(),
        None => "".to_string()
    };

    // Set new csrf cookie
    cookies.add_private(Cookie::new(".csrf", new_csrf_cookie.b64_string()));

    drop(cookies);

    // TODO: unwrap
    let token_bytes = match BASE64.decode(mail_data._csrf.as_bytes()) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Failed to parse csrf token. Token not Base64.");
            context.message = String::from("CSRF validation failed. Try again.");
            return Err(Template::render("index", &context))
        }
    };

    let parsed_token = match protect.parse_token(&token_bytes) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Failed to parse csrf token bytes.");
            context.message = String::from("CSRF validation failed. Try again.");
            return Err(Template::render("index", &context))
        }
    };

    // TODO: unwrap
    let cookie_bytes = match BASE64.decode(csrf_cookie.as_bytes()) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Failed to parse csrf cookie. Cookie not Base64.");
            context.message = String::from("CSRF validation failed. Try again.");
            return Err(Template::render("index", &context))
        }
    };

    let parsed_cookie = match protect.parse_cookie(&cookie_bytes) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Failed to parse csrf cookie bytes.");
            context.message = String::from("CSRF validation failed. Try again.");
            return Err(Template::render("index", &context))
        }
    };

    if !protect.verify_token_pair(&parsed_token, &parsed_cookie) {
        eprintln!("Failed to verify csrf token pair.");
        context.message = String::from("CSRF validation failed. Try again.");
        return Err(Template::render("index", &context))
    }

    /*
    *  Form interactive?
    */
    if !mail_data._interactive {
        context.message = String::from("Site not interacted with.");
        return Err(Template::render("index", &context))
    }

    /*
    *  Validate form data.
    */
    let (valid, errors) = mail_data.validate();
    if !valid {
        context.message = String::from("Invalid form data:");
        context.alerts.append(&mut errors.clone());
        return Err(Template::render("index", &context))
    }

    /*
    *  Send the email.
    */
    if !mailer::send(&mail_data) {
        context.message = String::from("Sending message failed. Try agian.");
        return Err(Template::render("index", &context))
    }

    Ok(Flash::success(RawRedirect((), Location(String::from("/#contact"))), "Message sent!"))

}
