mod lib;
use lib::tokenizer::tokenize as tokenize;

fn main() {
    match tokenize("test/main.ty"){
        Ok(tokens) => {
            for token in tokens.iter(){
                println!("{}", token.token);
            }
        }
        Err(_e) => {
            println!("Error");
        }
    };
}
