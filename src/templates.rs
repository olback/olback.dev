/*
 *  olback.net web server
 */

#[derive(Serialize)]
pub struct IndexTC {
    pub class: String,
    pub message: String
}

#[derive(Serialize)]
pub struct ErrorTemplate {
    pub code: i16,
    pub message: String
}