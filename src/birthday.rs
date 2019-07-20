use conf::{self, BirthdayConfig};
extern crate chrono;
use self::chrono::{Utc, Datelike};

/*
 *  BirthdayConfig
 */
static mut BDAY_CONF: Option<BirthdayConfig> = None;

fn get_bday_conf() -> BirthdayConfig {

    unsafe {

        return match BDAY_CONF {
            Some(v) => v,
            None => {
                let bday_conf = conf::read_config().birthday;
                BDAY_CONF = Some(bday_conf);
                BDAY_CONF.unwrap()
            }

        }

    }

}

fn now_as_birthday_config() -> BirthdayConfig {

    let now = Utc::now();

    BirthdayConfig {
        year: now.year() as u16,
        month: now.month() as u8,
        day: now.day() as u8
    }

}

pub fn get_age() -> u8 {

    let bday = get_bday_conf();
    let now = now_as_birthday_config();

    if now.month > bday.month || (now.month == bday.month && now.day >= bday.day) {
        return (now.year - bday.year) as u8;
    } else {
        return ((now.year - bday.year) - 1) as u8
    }

}
