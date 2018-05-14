/*
 *  olback.net web server
 */

#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;
extern crate lettre;
extern crate lettre_email;
#[macro_use] extern crate serde_derive;

use std::path::{Path, PathBuf};
use rocket::response::{NamedFile, Redirect};
use rocket::request::{Form, Request};
use rocket_contrib::{Template};

mod mail;
mod conf;
mod site;

#[get("/")]
fn index() -> Template {
    Template::render("index", 0)
}

#[get("/assets/<file..>")]
fn assets(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).ok()
}

#[get("/download/<file..>")]
fn download(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("download/").join(file)).ok()
}

#[get("/mail")]
fn mail() -> Redirect {
    Redirect::to("/#contact")
}

#[get("/mail/success")]
fn success() -> Template {
    let context = site::IndexTC {
        class: "success".to_string(),
        message: "Message sent!".to_string()
    };

    Template::render("index", &context)
}

// TODO: Refill form on failure
#[get("/mail/error")]
fn error() -> Template {
    let context = site::IndexTC {
        class: "error".to_string(),
        message: "Message could not be sent. Make sure all fields are filled in or try again. Please try again or send an email to contact@olback.net.".to_string()
    };

    Template::render("index", &context)
}

// TODO: recaptcha!
#[post("/mail", data = "<mail>")]
fn send_mail(mail: Form<mail::Mail>) -> Redirect {
    let mail_data = mail.into_inner();

    if !site::check_form_data(&mail_data) {
        return Redirect::to("/mail/error#contact");
    }

    if mail::send(mail_data) {
        return Redirect::to("/mail/success#contact");
    }

    return Redirect::to("/mail/error#contact")
}

// TODO: Make a 404 page
#[error(404)]
fn not_found(req: &Request) -> Template {
    let context = site::ErrorTemplate {
        code: 404,
        message: format!("The path {} could not be found.", req.uri())
    };

    Template::render("error", &context)
}

#[error(500)]
fn internal_server_error() -> Template {
    let context = site::ErrorTemplate {
        code: 500,
        message: "The server encountered an internal error while processing this request.".to_string()
    };

    Template::render("error", &context)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, assets, download, mail, success, error, send_mail])
    .attach(Template::fairing())
    .catch(errors![not_found, internal_server_error])
    .launch();
}