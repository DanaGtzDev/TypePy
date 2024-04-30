mod lib;
use lib::tokenizer::tokenize as tokenize;

fn main() {
    match tokenize("test/main.ty"){
        Ok(tokens) => {
            println!("WIJIU");
        }
        Err(e) => {
            println!("Error");
        }
    };
}
