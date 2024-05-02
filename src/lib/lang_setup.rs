use serde::{Deserialize, Serialize};
use std::io::Result;
use crate::lib::file_to_string as reader;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LangConfig {
    pub regex: String,
    pub tags: Vec<String>,
}


pub fn config(filename: &str) -> Result<Vec<LangConfig>> {
    match reader::read(filename) {
        Ok(content) => {
            let json = content.as_str();
            let tokens: Vec<LangConfig> = serde_json::from_str(json)?;
            Ok(tokens)
        }
        Err(e) => Err(e),
    }
}
