mod constant;
mod credential;
mod mail;
mod token;

use read_input::prelude::*;
use std::io::Error;
use zxcvbn::zxcvbn;

use credential::Credential;
use token::Token;

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

fn change_2fa_handle() -> Result<(), Error> {
    // copy pasted from login because i need to know email input at this level

    let email_message = "Please enter an email address: ";
    let mut email_input;
    loop {
        email_input = get_string_unrestricted(email_message);
        if !mail::is_email_valid(email_input.as_str()) {
            println!("Your email is not a real email address.");
            continue;
        }
        break;
    }

    let password_message = "Please enter a password: ";
    let mut password_input;
    loop {
        password_input = get_string_unrestricted(password_message);
        if password_input.len() > 64 {
            println!("password too long.");
            continue;
        }
        break;
    }

    match Credential::is_2fa_active(email_input.as_str()) {
        Ok(true) => {
            let google_token_message = "Please enter a google auth token: ";
            let mut google_auth_token_input;
            loop {
                google_auth_token_input = get_string_unrestricted(google_token_message);
                if google_auth_token_input.len() > 6 {
                    println!("token too long.");
                    continue;
                }
                break;
            }

            if !(Credential::is_verified_with_2fa(
                email_input.as_str(),
                google_auth_token_input.as_str(),
            ) && Credential::is_verified_with_password(
                email_input.as_str(),
                password_input.as_str(),
            )) {
                println!("{}", constant::AUTH_FAILED);
                return Ok(());
            };
        }
        Ok(false) => {
            if !Credential::is_verified_with_password(email_input.as_str(), password_input.as_str())
            {
                println!("{}", constant::AUTH_FAILED);
                return Ok(());
            };
        }
        Err(e) => return Err(e),
    }

    // is logged do stuff

    Credential::change_2fa_in_db(email_input.as_str())?;

    println!("If you forgot you google auth qr code, we sent it a long time ago at your email.\nIf you deleted it contact an administrator.");

    Ok(())
}

fn handle_login() -> Result<bool, Error> {
    let email_message = "Please enter an email address: ";
    let mut email_input;
    loop {
        email_input = get_string_unrestricted(email_message);
        if !mail::is_email_valid(email_input.as_str()) {
            println!("Your email is not a real email address.");
            continue;
        }
        break;
    }

    let password_message = "Please enter a password: ";
    let mut password_input;
    loop {
        password_input = get_string_unrestricted(password_message);
        if password_input.len() > 64 {
            println!("password too long.");
            continue;
        }
        break;
    }

    match Credential::is_2fa_active(email_input.as_str()) {
        Ok(true) => {
            let google_token_message = "Please enter a google auth token: ";
            let mut google_auth_token_input;
            loop {
                google_auth_token_input = get_string_unrestricted(google_token_message);
                if google_auth_token_input.len() > 6 {
                    println!("token too long.");
                    continue;
                }
                break;
            }

            return Ok(Credential::is_verified_with_2fa(
                email_input.as_str(),
                google_auth_token_input.as_str(),
            ) && Credential::is_verified_with_password(
                email_input.as_str(),
                password_input.as_str(),
            ));
        }
        Ok(false) => {
            return Ok(Credential::is_verified_with_password(
                email_input.as_str(),
                password_input.as_str(),
            ));
        }
        Err(e) => return Err(e),
    }
}

fn handle_token_redirection(all_credentials: &mut Vec<Credential>) -> Result<(), Error> {
    let all_tokens: Vec<Token>;

    match Token::collect_all_tokens() {
        Ok(tokens) => all_tokens = tokens,
        Err(_) => all_tokens = Vec::new(),
    };

    let token_message = "Please enter the token you recieved via email: ";
    let b64_token = get_string_unrestricted(token_message);

    let my_token: &Token;

    match all_tokens
        .iter()
        .find(|&token| token.b64_token == b64_token)
    {
        Some(token) => my_token = token,
        None => {
            println!("{}", constant::INVALID_TOKEN);
            return Ok(());
        }
    }

    if !my_token.is_token_valid() {
        println!("{}", constant::INVALID_TOKEN);
        return Ok(());
    }

    let mut email_exists = false;
    if all_credentials
        .iter()
        .any(|cred| cred.email == my_token.initiator_email)
    {
        email_exists = true;
    }

    if email_exists {
        // we can move to real pass reset

        match handle_pass_reset(my_token) {
            Ok(_) => println!("your password has been reset, try to login now."),
            Err(_) => println!("the password process failed."),
        }

        // handle forgot password
    } else {
        // we can move to real registration
        match handle_register(my_token) {
            Ok(_) => println!("try to login now."),
            Err(_) => println!("the registration process failed."),
        }
    }

    match my_token.delete() {
        Ok(_) => println!(
            "Your token expired, if your operation wasn't successful, you will need a new one."
        ),
        Err(_) => panic!("Token could not be deleted, contact an administrator."),
    }

    Ok(())
}

fn handle_pass_reset(token: &Token) -> Result<(), Error> {
    println!(
        "Welcome back.\nPlease proceed with the password reset for {}.",
        token.initiator_email
    );

    match Credential::is_2fa_active(token.initiator_email.as_str()) {
        Ok(true) => {
            let google_token_message = "Please enter a google auth token: ";
            let mut google_auth_token_input: String;

            loop {
                google_auth_token_input = get_string_unrestricted(google_token_message);
                if google_auth_token_input.len() > 6 {
                    println!("token too long.");
                    continue;
                }
                break;
            }

            if !Credential::is_verified_with_2fa(
                token.initiator_email.as_str(),
                google_auth_token_input.as_str(),
            ) {
                println!("Invalid google auth, password reset rejected.");
                return Ok(());
            }
        }
        Ok(false) => (), // do nothing, no 2fa no problems
        Err(e) => return Err(e),
    }

    let new_password_message = "Please enter your new password: ";
    let mut new_password_input: String;

    let confirm_new_password_message = "Please confirm your new password: ";
    let mut confirm_new_password_input: String;

    loop {
        new_password_input = get_string_unrestricted(new_password_message);
        confirm_new_password_input = get_string_unrestricted(confirm_new_password_message);

        if new_password_input.len() > 64 || confirm_new_password_input.len() > 64 {
            println!("one or both entries are too long");
            continue;
        }

        if new_password_input != confirm_new_password_input {
            println!("passwords dont match.");
            continue;
        }

        if zxcvbn(new_password_input.as_str(), &[]).unwrap().score() < 3 {
            println!("password is too weak.");
            continue;
        }

        Credential::change_password_in_db(
            token.initiator_email.as_str(),
            new_password_input.as_str(),
        )?;

        break;
    }

    Ok(())
}

fn handle_register(token: &Token) -> Result<(), Error> {
    println!(
        "Welcome back.\nPlease proceed with the registration for {}.",
        token.initiator_email
    );

    loop {
        let password_message = "Please enter a password: ";
        let confirm_password_message = "Please confirm your password: ";

        let password = get_string_unrestricted(password_message);
        let confirmed_password = get_string_unrestricted(confirm_password_message);

        if password.len() > 64 || confirmed_password.len() > 64 {
            println!("one or both entries are too long");
            continue;
        }

        if password != confirmed_password {
            println!("passwords dont match");
            continue;
        }

        if zxcvbn(password.as_str(), &[]).unwrap().score() < 3 {
            println!("password is too weak");
            continue;
        }

        Credential::add_new_credential_in_db(token.initiator_email.as_str(), password.as_str())?;

        break;
    }

    println!("Your account has been registered.\nWe sent you the qr code for 2fa via mail, it is mandatory for first login, you can disable it afterwards\nWelcome to the family.");

    Ok(())
}

fn give_forgot_password_token(all_credentials: &mut Vec<Credential>) -> Result<(), Error> {
    let ask_token_message =
        "Please enter your email address, more instructions will be sent there: ";

    let email_input = get_string_unrestricted(ask_token_message);

    if !mail::is_email_valid(email_input.as_str()) {
        println!("Not an email address.");
        return Ok(());
    }

    let mut is_already_in_use = false;

    if all_credentials.iter().any(|cred| cred.email == email_input) {
        is_already_in_use = true;
    }

    let mail_message: String = format!(
        "Here is your password reset token.\nIf this mail wasn't expected, please ignore it.\n{}",
        Token::create(email_input.as_str())
    ); // you can fill our db with token, token that wont ever be sent, and if you somehow mange to get it, you will just be able to create an account
       // I will not put the token creation in an if loop, as it makes it vulnerable to timing attack, it's maybe a bad idea, but who cares

    if is_already_in_use {
        mail::send_mail_to(
            email_input.as_str(),
            format!("Password reset to {}", constant::WEBSITE_NAME).as_str(),
            mail_message.as_str(),
        )?;
    } // google auth will be done when pass needs to change, someone could've stolen link

    Ok(())
}

fn give_register_token(all_credentials: &mut Vec<Credential>) -> Result<(), Error> {
    let ask_token_message = "Please enter an email address, more instructions will be sent there: ";

    let email_input = get_string_unrestricted(ask_token_message);

    if !mail::is_email_valid(email_input.as_str()) {
        println!("Not an email address");
        return Ok(());
    }

    let mut is_already_in_use = false;

    if all_credentials.iter().any(|cred| cred.email == email_input) {
        is_already_in_use = true;
    }

    let mail_message: String;

    if is_already_in_use {
        mail_message = String::from(format!(
            "You are already registered\nIf this message wasn't expected, please ignore it."
        ));
    } else {
        mail_message = String::from(format!(
            "Here is your token:\n{}",
            Token::create(email_input.as_str())
        ))
    }

    mail::send_mail_to(
        email_input.as_str(),
        format!("Registration to {}", constant::WEBSITE_NAME).as_str(),
        mail_message.as_str(),
    )?;

    // in practice there is no return at all, you are stuck on the webpage if the mail was successfull or not.

    Ok(())
}

fn main() {
    let message_list = "1:\tLogin\n2:\tEnter a Token\n3:\tGet password reset token\n4:\tGet register token\n5:\tChange 2FA status\nEnter your choice: ";
    let mut is_logged = false;
    loop {
        let mut all_cred: Vec<Credential>;
        match Credential::collect_all_credentials() {
            Ok(vec_cred) => all_cred = vec_cred,
            Err(_) => all_cred = Vec::new(), // db empty, no need to panic
        }

        Token::delete_old_token().unwrap(); // panics if this fails

        println!("Welcome to {}\n", constant::WEBSITE_NAME);
        println!("Logged status: {}", is_logged);
        let choice_input = get_choice(5, message_list);

        match choice_input {
            1 => match handle_login() {
                Ok(my_bool) => is_logged = my_bool,
                Err(_) => println!("{}", constant::AUTH_FAILED),
            },
            2 => match handle_token_redirection(&mut all_cred) {
                Ok(_) => (),
                Err(e) => println!("Error happened: {}", e),
            },
            3 => match give_forgot_password_token(&mut all_cred) {
                Ok(_) => (),
                Err(e) => println!("Error happened: {}", e),
            },
            4 => match give_register_token(&mut all_cred) {
                Ok(_) => (),
                Err(e) => println!("Error happened: {}", e),
            },
            5 => match change_2fa_handle() {
                Ok(_) => (),
                Err(e) => println!("Error happened: {}", e),
            },
            _ => panic!("No, that is illegal you know ?"),
        }
    }
}
