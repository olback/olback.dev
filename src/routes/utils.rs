/*
 *  olback.net web server
 */

use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};
use last_git_commit::{LastGitCommit, Id};
use rocket::{get, post, response::status, http::Status};
use openssl::sha;
use hex;
use std::{
    fs::File,
    io::prelude::*
};
use super::super::conf;

#[derive(Serialize)]
pub struct Version {
    version: String
}

impl Version {
    pub fn new() -> Self {
        let lgc = LastGitCommit::new(None, Some("4.0")).unwrap();
        Version {
            version: lgc.id.short()
        }
    }
}

#[get("/version")]
pub fn version() -> Json<Version> {
    Json(Version::new())
}

#[derive(Deserialize, Debug)]
pub struct Update {
    build_num: String,
    node_index: String
}

#[derive(Serialize)]
pub struct UpdateResponse {
    message: String,
    errors: Vec<&'static str>
}

#[post("/update", format = "json", data = "<update_json>")]
pub fn update(update_json: Json<Update>) -> status::Custom<Json<UpdateResponse>> {

    let update_data = update_json.into_inner();
    let mut errors = Vec::<&str>::new();

    let urls = conf::read_config().circle_ci.get_urls(update_data.build_num, update_data.node_index);

    // Binary blob
    let mut b_res = reqwest::get(&urls.blob).unwrap();
    if b_res.status() != 200 {
        errors.push("Error fetching binary blob");
    }
    let mut blob = Vec::<u8>::new();
    b_res.copy_to(&mut blob).unwrap();

    // Checksum
    let mut c_res = reqwest::get(&urls.checksum).unwrap();
    if c_res.status() != 200 {
        errors.push("Error fetching checksum");
    }
    let c_data = c_res.text().unwrap_or("".to_string());
    let checksum = c_data.split("  ").next().unwrap_or("").to_string();
    let checksum_calculated = hex::encode(sha::sha256(&blob));

    if checksum == checksum_calculated {
        // Git pull to update assets
        match std::process::Command::new("git").arg("pull").spawn() {
            Ok(_) => {},
            Err(_) => {
                errors.push("git pull failed");
            }
        }
        // Attempt to write file to disk
        match File::create("target/release/olback_net") {
            Ok(mut buffer) => {
                buffer.write(&blob).unwrap();
            },
            Err(_) => {
                errors.push("Error writing blob to disk");
            }
        }
    } else {
        errors.push("Checksums does not match!");
    }

    status::Custom(if &errors.len() > &0 { Status:: InternalServerError } else { Status::Ok }, Json(UpdateResponse {
        message: if &errors.len() > &0 { String::from("Internal Server Error") } else { String::from("Ok") },
        errors: errors
    }))

}
