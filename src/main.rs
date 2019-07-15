/*
 *  olback.net web server
 */

#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket_contrib;
#[macro_use] extern crate rocket;
extern crate colored;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate csrf;
extern crate data_encoding;

mod mailer;
mod conf;
mod form;
mod templates;
mod raw_redirect;
mod flash_res;

use std::path::{Path, PathBuf};
use std::process;
use rocket::response::{NamedFile, Flash/*, Redirect*/};
use rocket::request::{Form, Request, FlashMessage};
use rocket::http::hyper::header::Location;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::templates::Template;
use colored::*;
use flash_res::FlashRes;
use raw_redirect::RawRedirect;
use csrf::{AesGcmCsrfProtection, CsrfProtection};
use data_encoding::BASE64;
use form::Validate as FormValidate;
use conf::Validate as ConfValidate;


#[get("/")]
fn index(flash: Option<FlashMessage>, mut cookies: Cookies) -> Template {

    let protect = AesGcmCsrfProtection::from_key(conf::get_aes_key());

    // Generate token/cookie pair
    let (csrf_token, csrf_cookie) = protect.generate_token_pair(None, 18000)
        .expect("couldn't generate token/cookie pair");

    // Set cookie
    cookies.add_private(Cookie::new(".csrf", csrf_cookie.b64_string()));

    // Construct context
    let mut context = templates::IndexTemplate {
        csrf: csrf_token.b64_string(),
        ..Default::default()
    };

    if flash.is_some() {

        // Drop cookies to prevent errors.
        drop(cookies);

        // println!("Flash response");

        let flash_res = flash.map(|msg| FlashRes {
                name: msg.name().to_string(),
                msg: msg.msg().to_string()
        }).unwrap();

        context.class = flash_res.name;
        context.message = flash_res.msg;

        return Template::render("index", &context)

    } else {

        return Template::render("index", &context)

    }
}

#[get("/assets/<file..>")]
fn assets(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).ok()
}

#[get("/download/<file..>")]
fn download(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("download/").join(file)).ok()
}

#[get("/<file..>", rank = 3)]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/contact")]
fn contact() -> RawRedirect {
    // Redirect::to("/#contact")
    RawRedirect((), Location(String::from("/#contact")))
}

#[post("/mail", data = "<mail>")]
fn send_mail(mail: Form<form::Mail>, mut cookies: Cookies) -> Flash<RawRedirect> {

    let mail_data = mail.into_inner();

    /*
     *  csrf check
     */
    let csrf_cookie = match cookies.get_private(".csrf") {
        Some(v) => v.value().to_string(),
        None => "".to_string()
    };

    let protect = AesGcmCsrfProtection::from_key(conf::get_aes_key());

    let token_bytes = match BASE64.decode(mail_data._csrf.as_bytes()) {
        Ok(v) => v,
        Err(_) => {
            // eprintln!("Failed to parse csrf token. Token not Base64.");
            return Flash::error(RawRedirect((), Location(String::from("/#contact"))), "Failed to parse csrf token. Please try again.");
        }
    };

    let parsed_token = match protect.parse_token(&token_bytes) {
        Ok(v) => v,
        Err(_) => {
            // eprintln!("Failed to parse csrf token bytes.");
            return Flash::error(RawRedirect((), Location(String::from("/#contact"))), "Failed to parse csrf token. Please try again.");
        }
    };

    let cookie_bytes = match BASE64.decode(csrf_cookie.as_bytes()) {
        Ok(v) => v,
        Err(_) => {
            // eprintln!("Failed to parse csrf cookie. Cookie not Base64.");
            return Flash::error(RawRedirect((), Location(String::from("/#contact"))), "Invalid csrf cookie. Please try again.");
        }
    };

    let parsed_cookie = match protect.parse_cookie(&cookie_bytes) {
        Ok(v) => v,
        Err(_) => {
            // eprintln!("Failed to parse csrf cookie bytes.");
            return Flash::error(RawRedirect((), Location(String::from("/#contact"))), "Invalid csrf cookie. Please try again.");
        }
    };

    if !protect.verify_token_pair(&parsed_token, &parsed_cookie) {
        return Flash::error(RawRedirect((), Location(String::from("/#contact"))), "csrf verification failed. Try again!")
    }

    /*
     *  Form interactive?
     */
    if !mail_data._interactive {
        return Flash::error(RawRedirect((), Location(String::from("/#contact"))), "Form not activated, bot?")
    }

    /*
     *  Validate form data.
     */
    if !mail_data.validate() {
        // return Redirect::to("/mail/error#contact");
        return Flash::error(RawRedirect((), Location(String::from("/#contact"))), "Form data invalid.")
    }

    /*
     *  Send the email.
     */
    if mailer::send(mail_data) {
        // return Redirect::to("/mail/success#contact");
        return Flash::success(RawRedirect((), Location(String::from("/#contact"))), "Message sent!")
    }

    // Redirect::to("/mail/error#contact")
    Flash::error(RawRedirect((), Location(String::from("/#contact"))), "Failed to send email.")

}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let context = templates::ErrorTemplate {
        code: 404,
        message: format!("The path {} {} could not be found.", req.method(), req.uri())
    };

    Template::render("error", &context)
}

#[catch(422)]
fn unprocessable_entity() -> Template {
    let context = templates::ErrorTemplate {
        code: 422,
        message: "Unprocessable Entity. The request was well-formed but was unable to be followed due to semantic errors.".to_string()
    };

    Template::render("error", &context)
}

#[catch(500)]
fn internal_server_error() -> Template {
    let context = templates::ErrorTemplate {
        code: 500,
        message: "The server encountered an internal error while processing this request.".to_string()
    };

    eprintln!("Internal Server Error (500)");
    Template::render("error", &context)
}

fn main() {

    if !conf::read_mail_config().validate() {
        eprintln!("{}", "Aborting, mail config not valid!".bold().red());
        process::exit(-1);
    }

    rocket::ignite()
    .mount("/", routes![index, assets, download, static_files, contact, send_mail])
    .attach(Template::fairing())
    .register(catchers![not_found, unprocessable_entity, internal_server_error])
    .launch();

}
