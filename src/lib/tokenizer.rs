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

use crate::lib::file_to_string::read;
use crate::lib::lang_setup::config;
use crate::lib::lang_setup::LangConfig;
use regex::Regex;
use std::io::Result;

pub struct Token {
    pub token: String,
    pub token_type: String,
}

fn token_generator(line: &str, conf: &Vec<LangConfig>) -> Vec<Token> {
    let mut scoped_tokens: Vec<Token> = Vec::new();
    for config in conf.iter() {
        let re = Regex::new(config.regex.as_str()).unwrap();
        if let Some(caps) = re.captures(line){
            for tag in config.tags.clone(){
                let new_token = Token {
                    token: caps[tag.as_str()].to_string(),
                    token_type: tag
                };
                scoped_tokens.push(new_token);
            }
        }else{

        }
        
    }
    //TODO Tokenizer
    return scoped_tokens;
}

pub fn tokenize(filename: &str) -> Result<Vec<Token>> {
    match config("language.json") {
        Ok(conf) => match read(filename) {
            Ok(content) => {
                let mut tokens: Vec<Token> = Vec::new();
                let lines = content.split("\\n");
                for line in lines {
                    let mut new_tokens = token_generator(line, &conf);
                    tokens.append(&mut new_tokens);
                }
                Ok(tokens)
            }
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
