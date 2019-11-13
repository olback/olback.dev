use reqwest;
use rocket::{
    request::{Request, FromRequest, Outcome},
    outcome::Outcome::*
};
use std::net::IpAddr;
use serde::Deserialize;
use super::super::conf;

#[derive(Debug, Deserialize)]
pub struct AbuseIpDbResponse {
    pub data: AbuseIpDbResponseData
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct AbuseIpDbResponseData {
    // pub ipAddress: String,
    // pub isPublic: bool,
    // pub ipVersion: u8,
    // pub isWhitelisted: bool,
    pub abuseConfidenceScore: u8,
    // pub countryCode: String,
    // pub countryName: String,
    // pub usageType: String,
    // pub isp: String,
    // pub domain: String,
    // pub totalReports: u32,
    // pub numDistinctUsers: u32,
    // pub lastReportedAt: String,
    // pub reports: Option<Vec<AbuseIpDbResponseDataReports>>
}

// #[allow(non_snake_case)]
// #[derive(Debug, Deserialize)]
// pub struct AbuseIpDbResponseDataReports {
//     pub reportedAt: String,
//     pub comment: String,
//     pub categories: Vec<u16>,
//     pub reporterId: u32,
//     pub reporterCountryCode: String,
//     pub reporterCountryName: String
// }

#[derive(Debug)]
pub struct AbuseIpDb(bool);

impl<'a, 'r> FromRequest<'a, 'r> for AbuseIpDb {

    // type Error = std::convert::Infallible;
    // return Failure((rocket::http::Status::new(403, "Forbidden"), String::from("No Ipv4")))

    type Error = Self;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {

        let config = conf::read_config();

        let ip_address = match request.remote().unwrap().ip() {
            IpAddr::V4(v) => format!("{}", v),
            IpAddr::V6(v) => format!("{}", v)
        };

        let client = reqwest::Client::new();
        let res = client
        .get(&format!("https://api.abuseipdb.com/api/v2/check?ipAddress={}", ip_address))
        .header("Key", config.abuse_ip_db.token)
        .send();

        let res_data: AbuseIpDbResponse = res.unwrap().json().unwrap();

        if res_data.data.abuseConfidenceScore > 30 {

            Failure((rocket::http::Status::new(403, "Forbidden"), AbuseIpDb(false)))

        } else {

            Success(AbuseIpDb(true))

        }

    }

}
