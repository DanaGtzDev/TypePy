use crate::lib::file_to_string::read;
use crate::lib::lang_setup::config;
use crate::lib::lang_setup::LangConfig;
use regex::Regex;
use std::io::Result;

#[derive(Clone)]
pub struct Token {
    pub token: String,
    pub token_type: String,
}

fn array_tokenize(input: &str) -> Vec<Token> {
    let mut array_tokens: Vec<Token> = Vec::new();
    let token_pattern = Regex::new(
        r#"(?x)
        \s*,\s*                 # Commas
        |"[^"]*"                # Double-quoted strings, capturing quotes
        |\b[a-zA-Z_][a-zA-Z0-9_]*\b  # Identifiers
        |[0-9]+                 # Numbers
    "#,
    )
    .unwrap();

    let tp: Vec<String> = token_pattern
        .find_iter(input)
        .map(|mat| mat.as_str().trim().to_string())
        .collect();

    for token in tp.iter() {
        if token == "," {
            let new_token = Token {
                token: token.to_string(),
                token_type: "PUNCTUATION_COMA".to_string(),
            };
            array_tokens.push(new_token);
        } else {
            let new_token = Token {
                token: token.to_string(),
                token_type: "LITERAL".to_string(),
            };
            array_tokens.push(new_token);
        }
    }
    return array_tokens;
}

fn token_generator(line: &str, conf: &Vec<LangConfig>) -> Vec<Token> {
    let mut scoped_tokens: Vec<Token> = Vec::new();
    for config in conf.iter() {
        let re = Regex::new(config.regex.as_str()).unwrap();
        if let Some(caps) = re.captures(line) {
            for tag in config.tags.clone() {
                if tag == "LIST" || tag == "PARAMETERS" {
                    let tokens_array = array_tokenize(&caps[tag.as_str()]);
                    for ta in tokens_array.iter() {
                        scoped_tokens.push((*ta).clone());
                    }
                } else {
                    let new_token = Token {
                        token: caps[tag.as_str()].to_string(),
                        token_type: tag.trim_end_matches('_').to_string(),
                    };
                    scoped_tokens.push(new_token);
                }
            }
        } else {
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
                let lines = content.split("\n");

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
