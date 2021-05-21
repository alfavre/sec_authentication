use super::constant;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Credential {
    pub email: String,
    pub password: String,
    pub password_hash: String,
    pub salt: String,
}

impl Credential {
    pub fn get_default_cred() -> Credential {
        Credential {
            email: String::from("defaul@email.net"),
            password: String::from("defaultpass"),
            password_hash: String::from("defaulthash"),
            salt: String::from("defaultsalt"),
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
}
