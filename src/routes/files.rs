/*
 *  olback.net web server
 */

use std::path::{Path, PathBuf};
use rocket::{
    get,
    response::NamedFile
};

#[get("/assets/<file..>")]
pub fn assets(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).ok()
}

#[get("/download/<file..>")]
pub fn download(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("download/").join(file)).ok()
}

#[get("/<file..>", rank = 3)]
pub fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}