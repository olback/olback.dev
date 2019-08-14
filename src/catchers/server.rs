/*
 *  olback.net web server
 */

use rocket_contrib::templates::Template;
use templates;

#[catch(500)]
pub fn internal_error() -> Template {
    let context = templates::ErrorTemplate {
        code: 500,
        message: "The server encountered an internal error while processing this request.".to_string()
    };

    eprintln!("Internal Server Error (500)");
    Template::render("error", &context)
}
