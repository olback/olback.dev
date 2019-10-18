/*
 *  olback.net web server
 */

use colored::*;
use std::fs;
use serde::Deserialize;
use rand::Rng;

#[derive(Deserialize)]
pub struct Config {
    pub birthday: BirthdayConfig,
    pub mail: MailConfig,
    pub circle_ci: CircleCi
}

#[derive(Deserialize, Clone, Copy)]
pub struct BirthdayConfig {
    pub year: u16,
    pub month: u8,
    pub day: u8
}

#[derive(Deserialize)]
pub struct MailConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from: String,
    pub to: String,
    pub name: String,
    pub site: String
}

#[derive(Deserialize)]
pub struct CircleCi {
    pub username: String,
    pub repo_name: String,
    pub branch: String,
    pub token: String,
    pub blob: String,
    pub checksum: String
}

pub struct CircleCiUrl {
    pub blob: String,
    pub checksum: String
}

impl CircleCi {
    pub fn get_urls(&self, build_num: String, node_index: String) -> CircleCiUrl {
        let url = format!(
            "https://circleci.com/api/v1.1/project/github/{username}/{repo_name}/{build_num}/artifacts/{node_index}/:file/?branch={branch}&filter=successful&circle-token={token}",
            username = &self.username, repo_name = &self.repo_name, build_num = build_num, node_index = node_index, branch = &self.branch, token = &self.token
        );
        CircleCiUrl {
            blob: url.replace(":file", &self.blob),
            checksum: url.replace(":file", &self.checksum)
        }
    }
}

pub trait Validate {
    fn validate(&self) -> bool;
}

impl Validate for MailConfig {
    fn validate(&self) -> bool {
        let mut pass = true;

        if self.host.is_empty() {
            println!("{}", "Field 'host' is empty!".bold().yellow());
            pass = false;
        }

        if self.port == 0 {
            println!("{}", "Field 'port' is 0 (zero)!".bold().yellow());
            pass = false;
        }

        if self.username.is_empty() {
            println!("{}", "Field 'user' is empty!".bold().yellow());
            pass = false;
        }

        if self.password.is_empty() {
            println!("{}", "Field 'pass' is empty!".bold().yellow());
            pass = false;
        }

        if self.from.is_empty() {
            println!("{}", "Field 'from' is empty!".bold().yellow());
            pass = false;
        }

        if self.to.is_empty() {
            println!("{}", "Field 'to' is empty!".bold().yellow());
            pass = false;
        }

        if self.name.is_empty() {
            println!("{}", "Field 'name' is empty!".bold().yellow());
            pass = false;
        }

        if self.site.is_empty() {
            println!("{}", "Field 'site' is empty!".bold().yellow());
            pass = false;
        }

        pass
    }
}

pub fn read_config() -> Config {

    let conf_str = fs::read_to_string("Config.toml").unwrap_or_else(|e| {
        eprintln!("{} {}", "Config.toml not found.".bold().red(), e);
        format!("Config.toml not found. {}", e)
    });

    let config: Config = toml::from_str(conf_str.as_str()).unwrap();

    config

}

/*
 *  AES key for csrf protection
 */
static mut AES_KEY: Option<[u8; 32]> = None;

pub fn get_aes_key() -> [u8; 32] {

    unsafe {

        return match AES_KEY {
            Some(v) => v,
            None => {
                let key: [u8; 32] = rand::thread_rng().gen::<[u8; 32]>();
                AES_KEY = Some(key);
                AES_KEY.unwrap()
            }

        }

    }

}
