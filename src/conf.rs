/*
 *  olback.net web server
 */

extern crate rand;

use colored::*;
use std::fs;
use serde::Deserialize;
use self::rand::Rng;

#[derive(Deserialize)]
pub struct Config {
    pub birthday: BirthdayConfig,
    pub mail: MailConfig,
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
        eprintln!("{} {}", "MailConfig.toml not found.".bold().red(), e);
        format!("MailConfig.toml not found. {}", e)
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
