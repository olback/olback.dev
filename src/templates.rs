/*
 *  olback.net web server
 */

#[derive(Serialize)]
pub struct IndexTemplate {
    pub class: String,
    pub message: String
}

impl Default for IndexTemplate {
    fn default() -> IndexTemplate {
        IndexTemplate {
            class: String::from(""),
            message: String::from(""),
        }
    }
}

#[derive(Serialize)]
pub struct ErrorTemplate {
    pub code: u16,
    pub message: String
}
