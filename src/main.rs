/*
 *  olback.net web server
 */

#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket_contrib;
#[macro_use] extern crate rocket;
extern crate colored;
extern crate serde;
#[macro_use] extern crate serde_derive;

mod mail;
mod conf;
mod form;
mod templates;
mod raw_redirect;

use std::path::{Path, PathBuf};
use std::process;
use rocket::response::{NamedFile/*, Redirect*/};
use rocket::request::{Form, Request};
use rocket::http::hyper::header::Location;
use rocket_contrib::templates::Template;
use colored::*;
use raw_redirect::RawRedirect;

#[get("/")]
fn index() -> Template {
    Template::render("index", templates::IndexTemplate::default())
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

#[get("/mail/success")]
fn success() -> Template {
    let context = templates::IndexTemplate {
        class: "success".to_string(),
        message: "Message sent!".to_string()
    };

    Template::render("index", &context)
}

// TODO: Refill form on failure
#[get("/mail/error")]
fn error() -> Template {
    let context = templates::IndexTemplate {
        class: "error".to_string(),
        message: "Message could not be sent. Make sure all fields are filled in or try again. Please try again or send an email to contact@olback.net.".to_string()
    };

    Template::render("index", &context)
}

// TODO: recaptcha? Check referer header?
#[post("/mail", data = "<mail>")]
fn send_mail(mail: Form<mail::Mail>) -> RawRedirect {
    let mail_data = mail.into_inner();

    if !form::check_form_data(&mail_data) {
        // return Redirect::to("/mail/error#contact");
        return RawRedirect((), Location(String::from("/mail/error#contact")));
    }

    if mail::send(mail_data) {
        // return Redirect::to("/mail/success#contact");
        return RawRedirect((), Location(String::from("/mail/success#contact")));
    }

    // Redirect::to("/mail/error#contact")
    RawRedirect((), Location(String::from("/mail/error#contact")))
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

    Template::render("error", &context)
}

fn main() {

    if !conf::check_mail_config() {
        println!("{}", "Aborting, mail config not valid!".bold().red());
        process::exit(-1);
    }

    rocket::ignite()
    .mount("/", routes![index, assets, download, static_files, contact, success, error, send_mail])
    .attach(Template::fairing())
    .register(catchers![not_found, unprocessable_entity, internal_server_error])
    .launch();

}
