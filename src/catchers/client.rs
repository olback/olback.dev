/*
 *  olback.net web server
 */

use rocket_contrib::templates::Template;
use rocket::{
    catch,
    request::Request
};
use super::super::templates;

#[catch(400)]
pub fn bad_request() -> Template {
    let context = templates::ErrorTemplate {
        code: 400,
        message: format!("Bad request")
    };

    Template::render("error", &context)
}

#[catch(404)]
pub fn not_found(req: &Request) -> Template {
    let context = templates::ErrorTemplate {
        code: 404,
        message: format!("The path {} {} could not be found.", req.method(), req.uri())
    };

    Template::render("error", &context)
}

#[catch(422)]
pub fn unprocessable_entity() -> Template {
    let context = templates::ErrorTemplate {
        code: 422,
        message: "Unprocessable Entity. The request was well-formed but was unable to be followed due to semantic errors.".to_string()
    };

    Template::render("error", &context)
}
