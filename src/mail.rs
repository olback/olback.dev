/*
 *  olback.net web server
 */

#[derive(FromForm)]
pub struct Mail {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub body: String,
    pub copy: bool
}
