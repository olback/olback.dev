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
use rocket::response::{NamedFile, Flash/*, Redirect*/};
use rocket::request::{Form, Request, FlashMessage};
use rocket::http::hyper::header::Location;
// use rocket::http::{Cookie, Cookies};
use rocket_contrib::templates::Template;
use colored::*;
use raw_redirect::RawRedirect;

struct FlashRes {
    name: String,
    msg: String
}

#[get("/")]
fn index(flash: Option<FlashMessage>/*, mut cookies: Cookies*/) -> Template {

    // Make sure the CSRF private cookie is set.

    // let csrf = match cookies.get_private(".csrf") {
    //     Some(v) => v.value().to_string(),
    //     None => "".to_string()
    // };

    // println!(".csrf: {}", csrf);

    // if csrf.is_empty() {
    //     cookies.add_private(Cookie::new(".csrf", "test value"));
    // }

    if flash.is_some() {

        let flash_res = flash.map(|msg| FlashRes {
                name: msg.name().to_string(),
                msg: msg.msg().to_string()
        }).unwrap();

        Template::render("index", templates::IndexTemplate {
            class: flash_res.name,
            message: flash_res.msg,
            ..Default::default()
        })

    } else {

        Template::render("index", templates::IndexTemplate::default())

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
fn send_mail(mail: Form<mail::Mail>) -> Flash<RawRedirect> {

    let mail_data = mail.into_inner();

    if !mail_data._interactive {
        return Flash::error(RawRedirect((), Location(String::from("/#contact"))), "Form not activated, bot?")
    }

    // TODO: CSRF

    if !form::check_form_data(&mail_data) {
        // return Redirect::to("/mail/error#contact");
        return Flash::error(RawRedirect((), Location(String::from("/#contact"))), "Form data invalid.")
    }

    if mail::send(mail_data) {
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

    Template::render("error", &context)
}

fn main() {

    if !conf::check_mail_config() {
        println!("{}", "Aborting, mail config not valid!".bold().red());
        process::exit(-1);
    }

    rocket::ignite()
    .mount("/", routes![index, assets, download, static_files, contact, send_mail])
    .attach(Template::fairing())
    .register(catchers![not_found, unprocessable_entity, internal_server_error])
    .launch();

}
