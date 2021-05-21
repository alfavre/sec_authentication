use super::constants;
use std::fs::File;
use std::io::{Error, Write};
use chrono::prelude::*;


pub fn send_mail_to(address:&str,object:&str,message:&str) -> Result<(),Error>{

    let mut output = File::create(constants::MAILBOX_PATH)?;

    let local = Local::now();

    write!(output,"Date: {}\nFrom: admin@coolwebsite.com\nTo: {}\nObject: {}\nMessage:\n{}",local,address,object,message)?;

    Ok(())
}