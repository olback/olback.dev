/*
 *  olback.net web server
 */

#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)] // Feature will be depricated?

extern crate rocket;
extern crate rocket_contrib;

use std::io;
use std::path::{Path, PathBuf};
use rocket::response::{NamedFile, Redirect};
use rocket::request::{Form};

mod mail;
use mail::Mail;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
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
    Redirect::to("/")
}

#[post("/mail", data = "<_mail>")]
fn send_mail(_mail: Form<Mail>) ->  String {
    let mail_data = _mail.into_inner();

    if mail_data.name.is_empty() {
        return format!("Name may not be empty");
    }

    if mail_data.email.is_empty() {
        return format!("Email may not be empty");
    }

    if mail_data.subject.is_empty() {
        return format!("Subject may not be empty");
    }

    if mail_data.body.is_empty() {
        return format!("Mail body may not be empty");
    }


    format!("{} :: {} :: {} :: {} :: {}", mail_data.name, mail_data.email, mail_data.subject, mail_data.body, mail_data.copy)
}

#[error(404)]
fn not_found() -> io::Result<NamedFile> {
    NamedFile::open("static/404.html")
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, assets, download, mail, send_mail])
    .catch(errors![not_found])
    .launch();
}
