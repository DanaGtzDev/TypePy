#[allow(special_module_name)]
mod lib;
use lib::tokenizer::tokenize as tokenize;
use lib::tree_generator::tree_generator as tree;

fn main() {
    match tokenize("test/main.ty"){
        Ok(tokens) => {
            for token in tokens.iter(){
                println!("token: {} token_type: {}", token.token, token.token_type);
            }
            tree(tokens);
        }
        Err(_e) => {
            println!("Error");
        }
    };
}
