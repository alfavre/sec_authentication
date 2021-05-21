mod constant;
mod credential;
mod mail;
mod token;

use google_authenticator::GoogleAuthenticator;
use read_input::prelude::*;
use regex::Regex;
use std::io::Error;

use credential::Credential;

fn get_choice(numbers_of_choices: u32, choice_list: &str) -> u32 {
    input()
        .repeat_msg(choice_list) // for now, maybe do a dynamic struct "list" another day
        .err(format!(
            "Please enter a number in the range 1 to {}.\n",
            numbers_of_choices
        ))
        .add_test(move |x| *x <= numbers_of_choices && *x != 0)
        .get()
}

/// note, it is not verified here to stop verbose attacks
fn get_string_unrestricted(message: &str) -> String {
    input()
        .repeat_msg(message)
        .err("This entry is mandatory")
        .get()
}

fn handle_login() -> () {
    println!("log, TODO")
}

fn handle_register(all_credentials: &mut Vec<Credential>) -> Result<(), Error> {
    let email_message = "Please enter an email address, this will be your username: ";
    let password_message = "Please enter a password: ";
    let confirm_password_message = "Please confirm your password: ";

    let email = get_string_unrestricted(email_message);

    if !Regex::new(constant::MAIL_REGEX)
        .unwrap()
        .is_match(email.as_str())
    {
        println!("Invalid email");
        return Ok(());
    }

    let password = get_string_unrestricted(password_message);
    let confirmed_password = get_string_unrestricted(confirm_password_message);

    if password != confirmed_password {
        println!("passwords dont match");
        return Ok(());
    }

    // TODO verify password strength here with regex

    // we now have data
    // TODO hash data

    //we check if already exists
    //if all_credentials.contains(x: &T)

    all_credentials.push(Credential {
        email: email,
        password: password,
        password_hash: String::from("none"),
        salt: String::from("none"),
    });

    Credential::write_all_credentials(all_credentials)?;

    Ok(())
}

fn handle_forgot_password() -> () {
    println!("you frogot pass, TODO");
}

fn give_register_token(all_credentials: &mut Vec<Credential>) -> Result<(), Error> {
    let ask_token_message = "Please enter an email address, more instructions will be sent there: ";

    let email_input = get_string_unrestricted(ask_token_message);

    if !Regex::new(constant::MAIL_REGEX)
        .unwrap()
        .is_match(email_input.as_str())
    {
        println!("Invalid email");
        return Ok(());
    }

    let mut is_already_in_use = false;
    for cred in all_credentials {
        if cred.email == email_input {
            is_already_in_use = true;
        }
    }

    let mail_message: String;

    if is_already_in_use {
        mail_message = String::from(format!(
            "You are already registered\nIf this message wasn't expected, please ignore it."
        ));
    } else {
        mail_message = String::from(format!("Here is your token: {}",token::Token::create(email_input.as_str())))
    }

    mail::send_mail_to(
        email_input.as_str(),
        format!("Registration to {}", constant::WEBSITE_NAME).as_str(),
        mail_message.as_str(),
    )?;

    Ok(())
}

fn main() {
    let message_list = "1:\tlogin\n2:\tregister\n3:\tforgot my password\n4:\tget register token\nEnter your choice: ";

    token::Token::delete_old_token();

    loop {
        let mut all_cred = Credential::collect_all_credentials().unwrap();

        println!("Welcome to {}\n", constant::WEBSITE_NAME);
        let choice_input = get_choice(4, message_list);

        match choice_input {
            1 => handle_login(),
            2 => match handle_register(&mut all_cred) {
                Ok(_) => (),
                Err(e) => println!("Error happened: {}", e),
            },
            3 => handle_forgot_password(),
            4 => match give_register_token(&mut all_cred) {
                Ok(_) => (),
                Err(e) => println!("Error happened: {}", e),
            },
            _ => panic!("No, that is illegal you know ?"),
        }
    }
}
