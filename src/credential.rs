use super::{constant, mail};
use google_authenticator::{ErrorCorrectionLevel, GoogleAuthenticator};
use serde::{Deserialize, Serialize};
use sodiumoxide::base64::*;
use sodiumoxide::crypto::pwhash::HashedPassword;
use sodiumoxide::crypto::{pwhash, secretbox};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Credential {
    pub email: String,
    pub password_hash: String,
    pub google_authenticator_secret: String,
    pub is_2fa_active: bool,
}

impl Credential {
    fn new(email: &str, password: &str) -> Credential {
        let authenticator = GoogleAuthenticator::new();

        let my_secret = authenticator.create_secret(32);

        let qr_code_url = authenticator.qr_code_url(
            my_secret.as_str(),
            constant::WEBSITE_NAME,
            "2FA",
            0,
            0,
            ErrorCorrectionLevel::Medium,
        );

        match mail::send_mail_to(email, "Your 2fa qr code", format!("Here is your qr code link for 2FA.\nYou should activate 2FA on your phone as soon as possible,\nIt is required tp continue using our service.\n{}",qr_code_url).as_str()){
            Ok(_) => println!("The 2FA mail has been sent to {}",email),
            Err(_) => panic!("Mail could not be sent, sorry for the inconvenience."),
        }

        Credential {
            email: String::from(email),
            password_hash: encode(
                pwhash::pwhash(
                    password.as_bytes(),
                    pwhash::OPSLIMIT_INTERACTIVE,
                    pwhash::MEMLIMIT_INTERACTIVE,
                )
                .unwrap(),
                Variant::UrlSafe,
            ),
            google_authenticator_secret: my_secret,
            is_2fa_active: true,
        }
    }

    pub fn collect_all_credentials() -> Result<Vec<Credential>, Error> {
        let mut vec = Vec::new();

        let input = File::open(constant::DB_PATH)?;
        let buffered = BufReader::new(input);

        for line in buffered.lines() {
            vec.push(serde_json::from_str(line.unwrap().as_str()).unwrap());
        }
        Ok(vec)
    }

    pub fn write_all_credentials(all_cred: &mut Vec<Credential>) -> Result<(), Error> {
        let mut credentials_json = String::from("");
        let mut output = File::create(constant::DB_PATH)?;

        for cred in all_cred {
            credentials_json.push_str(&serde_json::to_string(&cred).unwrap());
            credentials_json.push_str("\n");
        }

        write!(output, "{}", credentials_json)?;

        Ok(())
    }

    /// it's impossible to create two credentials with the same email
    /// but maybe I should check anyway
    pub fn add_new_credential_in_db(email: &str, password: &str) -> Result<(), Error> {
        let mut all_credentials: Vec<Credential>;
        match Credential::collect_all_credentials() {
            Ok(vec_cred) => all_credentials = vec_cred,
            Err(_) => all_credentials = Vec::new(), // db empty, no need to panic
        }

        all_credentials.push(Credential::new(email, password));
        Credential::write_all_credentials(&mut all_credentials)?;
        Ok(())
    }

    pub fn is_2fa_active(email: &str) -> Result<bool, Error> {
        // we look if account exists
        let all_cred: Vec<Credential> = Credential::collect_all_credentials()?;

        match all_cred.iter().find(|&cred| cred.email == email) {
            Some(credential) => return Ok(credential.is_2fa_active),
            None => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("{}", constant::AUTH_FAILED),
                ))
            }
        }
    }

    pub fn is_verified_with_2fa(email: &str, code_2fa: &str) -> bool {
        // we look if account exists
        let all_cred: Vec<Credential>;
        match Credential::collect_all_credentials() {
            Ok(creds) => all_cred = creds,
            Err(_) => return false,
            // cant log if no cred in db
        }

        let the_secret_from_db: &String;
        match all_cred.iter().find(|&cred| cred.email == email) {
            Some(credential) => the_secret_from_db = &credential.google_authenticator_secret,
            None => return false,
        }

        let authenticator = GoogleAuthenticator::new();
        authenticator.verify_code(the_secret_from_db, code_2fa, 3, 0) // no idea what are those numbers

        // The parameter discrepancy indicates number of seconds ago that a code may be generated.
        // time_slice is used to modify what the current time is, as a unix timestamp.
        // If 0 is provided here, the current time will be used.
    }

    pub fn is_verified_with_password(email: &str, password: &str) -> bool {
        // we look if account exists
        let all_cred: Vec<Credential>;
        match Credential::collect_all_credentials() {
            Ok(creds) => all_cred = creds,
            Err(_) => return false,
            // cant log if no cred in db
        }

        let string_from_db: &String;
        match all_cred.iter().find(|&cred| cred.email == email) {
            Some(credential) => string_from_db = &credential.password_hash,
            None => return false,
        }

        let bizzaro_cast: &[u8] = &decode(string_from_db, Variant::UrlSafe).unwrap();

        // as I had to do the returns, it seems my code is particularly vulnerable to timing attack

        pwhash::pwhash_verify(
            &HashedPassword::from_slice(bizzaro_cast).unwrap(),
            password.as_bytes(),
        )
    }
}
