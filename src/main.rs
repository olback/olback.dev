/*
 *  olback.net web server
 */

#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
// use rocket::response::Redirect;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static_private/index.html")
}

#[get("/assets/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).ok()
}

#[error(404)]
fn e404() -> io::Result<NamedFile> {
    NamedFile::open("static_private/404.html")
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, files])
    // .mount("/assets/", routes![files])
    .catch(errors![e404])
    .launch();
}
