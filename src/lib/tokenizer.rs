/*

//use regex::Regex;
use serde::{Deserialize, Serialize};
//use std::fs::read_to_string;

#[derive(Debug, Deserialize, Serialize)]
struct Token {
    regex: String,
    tags: Vec<String>,
}
//pub fn variable_regex(line: &String)
pub fn variable_regex() {
    //"dummy/main.ty"
    /*
    language_definition_query("language.json");
    let re = Regex::new(r"(?<IDENTIFIER>[a-z_0-9]+)\s*(?<PUNCTUATOR_DOTDOT>:)\s*(?<TYPE>[a-z]+)\s*(?<PUNCTUATOR_EQUAL>=)\s*(?<LITERAL>[0-9A-Za-z'.\-]+)\s*").unwrap();
    let Some(caps) = re.captures(line) else {
        println!("INVALID SYNTAX");
        return;
    };
    println!("{}",&caps["IDENTIFIER"])
    */

    let json = r#"[{"regex": "(?<KEYWORD_TYPE>type)\\s+(?<IDENTIFIER>[A-Z][a-z]*)\\s*(?<PUNCTUATOR_DOTDOT>:)\\s*","tags":["KEYWORD_TYPE","IDENTIFIER","PUNCTUATOR_DOTDOT"]}]"#;
    let tokens: Vec<Token> = serde_json::from_str(json)?;
    //println!("{:?}", token[0].regex);
    Ok(());
}

*/

use crate::lib::lang_setup::config as config;
use crate::lib::file_to_string::read as read;
use crate::lib::lang_setup::LangConfig as LangConfig;
use std::io::Result;
use regex::Regex;


pub struct Token{
    pub token: String,
    pub token_type: String
}

fn token_generator(line: &str, conf: &Vec<LangConfig>) -> Vec<Token>{
    let mut scoped_tokens: Vec<Token> = Vec::new();
    
    //TODO Tokenizer
    return scoped_tokens;
}

pub fn tokenize(filename: &str) -> Result<Vec<Token>>{
    match config("language.json"){
        Ok(conf) => {
            match read(filename){
                Ok(content) => {
                    let mut tokens: Vec<Token> = Vec::new();
                    let lines = content.split("\\n");
                    for line in lines{
                        token_generator(line, &conf);
                    }   
                    Ok(tokens)
                }
                Err(e) => {
                    Err(e)
                }
            }
        }
        Err(e) => {
            Err(e)
        }
    }
    

    
}