use google_authenticator::GoogleAuthenticator;
use read_input::prelude::*;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
use serde::{Serialize, Deserialize};

// special thanks to : https://regexr.com/
const MAIL_REGEX: &'static str = r"^(?i)[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,4}$";
const DB_PATH: &'static str = "cooldatabase.txt";

#[derive(Serialize, Deserialize, Debug)]
struct Credential{
    email: String,
    password: String,
    password_hash: String,
    salt: String,
}

impl Credential{
    fn get_default_cred()->Credential{
        Credential{
            email: String::from("defaul@email.net"),
            password: String::from("defaultpass"),
            password_hash: String::from("defaulthash"),
            salt: String::from("defaultsalt"),
        }
    }

    fn collect_all_credentials()->Result<Vec<Credential>,Error>{

        let mut vec = Vec::new();
    
        let input = File::open(DB_PATH)?;
        let buffered = BufReader::new(input);
    
        for line in buffered.lines() {
            vec.push(serde_json::from_str(line.unwrap().as_str()).unwrap());
        }
        Ok(vec)
    }

    fn write_all_credentials(all_cred:&mut Vec<Credential>)->Result<(),Error>{

        let mut credentials_json = String::from("");
        let mut output = File::create(DB_PATH)?;

        for cred in all_cred {
            credentials_json.push_str(&serde_json::to_string(&cred).unwrap());
            credentials_json.push_str("\n");
        }

        write!(output,"{}",credentials_json)?;

        Ok(())

    }
}



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
    println!("log")
}

fn handle_register(all_credentials: &mut Vec<Credential>) -> Result<(), Error> {
    let email_message = "Please enter an email address, this will be your username: ";
    let password_message = "Please enter a password: ";
    let confirm_password_message = "Please confirm your password: ";

    let email = get_string_unrestricted(email_message);

    if !Regex::new(MAIL_REGEX).unwrap().is_match(email.as_str()) {
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

    all_credentials.push(Credential{
        email: email,
        password: password,
        password_hash: String::from("none"),
        salt: String::from("none"),
    });

    Credential::write_all_credentials(all_credentials)?;


    Ok(())
}



fn handle_forgot_password() -> () {
    println!("you frogot pass")
}

fn main() {
    let message_list = "1:\tlogin\n2:\tregister\n3:\tforgot my password\n\nEnter your choice: ";

    loop {

        
        let mut all_cred = Credential::collect_all_credentials().unwrap();
        
        println!("{:?}",&all_cred);
        

        println!("Welcome to secure auth dot com\n");
        let choice_input = get_choice(3, message_list);

        match choice_input {
            1 => handle_login(),
            2 => match handle_register(&mut all_cred) {
                Ok(_) => (),
                Err(e) => println!("Error happened: {}", e),
            },
            3 => handle_forgot_password(),
            _ => panic!("No, that is illegal you know ?"),
        }
    }
}
