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

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::io::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub regex: String,
    pub tags: Vec<String>,
}

fn read_file_to_string(filename: &str) -> Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn variable_regex() -> Result<Vec<Token>> {
    match read_file_to_string("language.json") {
        Ok(content) => {
            let json = content.as_str();

            let tokens: Vec<Token> = serde_json::from_str(json)?;
            Ok(tokens)
        }
        Err(e) => Err(e),
    }
}
