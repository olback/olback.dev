use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Url {
    pub display: String,
    pub url: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub name: String,
    pub color: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub body: String,
    pub git: Url,
    pub website: Option<Url>,
    pub language: Language
}

pub type Projects = Vec<Project>;

pub fn load_projects() -> Projects {

    if cfg!(debug_assertions) {
        let json_string = fs::read_to_string("Projects.json").unwrap();
        return serde_json::from_str(&json_string).unwrap();
    }

    return serde_json::from_str(include_str!("../Projects.json")).unwrap();

}


