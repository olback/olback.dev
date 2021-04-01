/*
 *  olback.net web server
 */

use chrono::{Utc, Datelike};
use serde::Serialize;
use super::{
    form,
    birthday,
    projects,
    social_media
};

#[derive(Serialize)]
pub struct IndexTemplate {
    pub class: String,
    pub message: String,
    pub alerts: Vec<String>,
    pub csrf: String,
    pub year: i32,
    pub age: u8,
    pub projects: projects::Projects,
    pub social_media: social_media::SocialMedia,
    pub mail: Option<form::Mail>
}

impl Default for IndexTemplate {
    fn default() -> IndexTemplate {
        IndexTemplate {
            class: String::from(""),
            message: String::from(""),
            alerts: Vec::new(),
            csrf: String::from(""),
            year: Utc::now().year(),
            age: birthday::get_age(),
            projects: projects::load_projects(),
            social_media: social_media::load_social_media(),
            mail: None
        }
    }
}

#[derive(Serialize)]
pub struct ErrorTemplate {
    pub code: u16,
    pub message: String
}
