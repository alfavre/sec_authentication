use super::constant;
use chrono::prelude::*;
use std::fs::File;
use std::io::{Error, Write};

pub fn send_mail_to(address: &str, object: &str, message: &str) -> Result<(), Error> {
    let mut output = File::create(constant::MAILBOX_PATH)?;

    let time = Utc::now();

    write!(
        output,
        "Date: {}\nFrom: {}\nTo: {}\nObject: {}\nMessage:\n{}",
        time,
        constant::WEBSITE_EMAIL,
        address,
        object,
        message
    )?;

    Ok(())
}
