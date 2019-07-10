/*
 *  olback.net web server
 */

use colored::*;
use std::fs;
use serde::Deserialize;

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

pub fn read_mail_config() -> MailConfig {

    let conf_str = fs::read_to_string("MailConfig.toml").unwrap_or_else(|e| {
        println!("{} {}", "MailConfig.toml not found.".bold().red(), e);
        format!("MailConfig.toml not found. {}", e)
    });

    let config: MailConfig = toml::from_str(conf_str.as_str()).unwrap();

    config

}

pub fn check_mail_config() -> bool {

    let mail_conf = read_mail_config();

    let mut pass = true;

    if mail_conf.host.is_empty() {
        println!("{}", "Field 'host' is empty!".bold().yellow());
        pass = false;
    }

    if mail_conf.port == 0 {
        println!("{}", "Field 'port' is 0 (zero)!".bold().yellow());
        pass = false;
    }

    if mail_conf.username.is_empty() {
        println!("{}", "Field 'user' is empty!".bold().yellow());
        pass = false;
    }

    if mail_conf.password.is_empty() {
        println!("{}", "Field 'pass' is empty!".bold().yellow());
        pass = false;
    }

    if mail_conf.from.is_empty() {
        println!("{}", "Field 'from' is empty!".bold().yellow());
        pass = false;
    }

    if mail_conf.to.is_empty() {
        println!("{}", "Field 'to' is empty!".bold().yellow());
        pass = false;
    }

    if mail_conf.name.is_empty() {
        println!("{}", "Field 'name' is empty!".bold().yellow());
        pass = false;
    }

    if mail_conf.site.is_empty() {
        println!("{}", "Field 'site' is empty!".bold().yellow());
        pass = false;
    }

    pass

}
