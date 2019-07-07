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
    pub user: String,
    pub pass: String,
    pub from: String,
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

    if mail_conf.user.is_empty() {
        println!("{}", "Field 'user' is empty!".bold().yellow());
        pass = false;
    }

    if mail_conf.pass.is_empty() {
        println!("{}", "Field 'pass' is empty!".bold().yellow());
        pass = false;
    }

    if mail_conf.from.is_empty() {
        println!("{}", "Field 'from' is empty!".bold().yellow());
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
