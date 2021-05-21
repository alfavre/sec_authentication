use super::constant;
use chrono::{Duration,Utc,DateTime};
use sodiumoxide::{base64::*,randombytes::*};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub b64_token: String,
    pub creation_time:DateTime<Utc>,
    pub initiator_email:String,
}

impl Token{
    pub fn create(initiator_email:&str)->String{

        let random_bytes:Vec<u8>=randombytes(32);
        let b64_endoded:String = encode(random_bytes, Variant::UrlSafe);
        let new_token = Token{
            b64_token: b64_endoded.clone(), // did not find how to do this without clone
            creation_time: Utc::now(),
            initiator_email: String::from(initiator_email),
        };


        match Token::add_new_token_in_db(new_token){
            Ok(_) => return b64_endoded,
            Err(_) => panic!("Something went wrong sorry"), // this may cause info leakage
        }
    }

    /// copy pasted from the similar fn from credential
    fn collect_all_tokens()->Result<Vec<Token>,Error>{

        let mut vec = Vec::new();

        let input = File::open(constant::TOKEN_PATH)?;
        let buffered = BufReader::new(input);

        for line in buffered.lines() {
            vec.push(serde_json::from_str(line.unwrap().as_str()).unwrap());
        }
        Ok(vec)
    }

    /// copy pasted from the similar fn from credential
    fn write_all_tokens(all_tokens: &mut Vec<Token>) -> Result<(), Error> {
        let mut tokens_json = String::from("");
        let mut output = File::create(constant::TOKEN_PATH)?;

        for token in all_tokens {
            tokens_json.push_str(&serde_json::to_string(&token).unwrap());
            tokens_json.push_str("\n");
        }

        write!(output, "{}", tokens_json)?;

        Ok(())
    }

    fn add_new_token_in_db(new_token:Token)->Result<(),Error>{

        let mut all_tokens: Vec<Token>;
        match Token::collect_all_tokens(){
            Ok(vec_tok) => all_tokens = vec_tok,
            Err(_) => all_tokens = Vec::new(), // db not created, no need to panic
        }

        
        all_tokens.push(new_token); // push doesn't want a ref for some reason
        Token::write_all_tokens(&mut all_tokens)?;
        Ok(()) // gives the token back, // could not manage this othewise sadly, lifetimes allways didnt work when deserialisation needs to happen
    }

    pub fn delete_old_token()-> Result<(),Error>{

        let mut all_tokens: Vec<Token>;
        match Token::collect_all_tokens(){
            Ok(vec_tok) => all_tokens = vec_tok,
            Err(_) =>  return Ok(()), // no token no work
        }


        println!("before {}",all_tokens.len());
        all_tokens.retain(|token|  Utc::now().signed_duration_since((*token).creation_time) < Duration::minutes(constant::TTL));
        println!("after {}",all_tokens.len());

        Token::write_all_tokens(&mut all_tokens);

        Ok(())
    }

}