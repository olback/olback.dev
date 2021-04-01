use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct SocialMediaItem {
    pub logo: String,
    pub label: String,
    pub url: String
}

pub type SocialMedia = Vec<SocialMediaItem>;

pub fn load_social_media() -> SocialMedia {

    if cfg!(debug_assertions) {
        let json_string = fs::read_to_string("SocialMedia.json").unwrap();
        return serde_json::from_str(&json_string).unwrap();
    }

    return serde_json::from_str(include_str!("../SocialMedia.json")).unwrap();

}


