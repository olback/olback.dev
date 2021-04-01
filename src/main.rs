/*
 *  olback.net web server
 */

#![feature(proc_macro_hygiene, decl_macro)]

mod mailer;
mod conf;
mod form;
mod templates;
mod raw_redirect;
mod birthday;
mod routes;
mod catchers;
mod guards;
mod projects;
mod social_media;

use std::process;
use rocket::{catchers, routes};
use rocket_contrib::templates::Template;
use colored::*;
use conf::Validate;

fn main() {

    if !conf::read_config().mail.validate() {
        eprintln!("{}", "Aborting, mail config not valid!".bold().red());
        process::exit(-1);
    }

    rocket::ignite()
    .mount("/", routes![
        routes::index,
        routes::files::assets,
        routes::files::download,
        routes::files::static_files,
        routes::redirects::mail_to_contact,
        routes::redirects::contact_to_contact,
        routes::mail::send_mail,
        routes::utils::version,
        routes::utils::update
    ])
    .attach(Template::fairing())
    .register(catchers![
        catchers::client::bad_request,
        catchers::client::forbidden,
        catchers::client::not_found,
        catchers::client::unprocessable_entity,
        catchers::server::internal_error
    ])
    .launch();

}
