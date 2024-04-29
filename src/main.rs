mod lib;

fn main() {
    match lib::lang_setup::variable_regex() {
        Ok(tokens) => {
            for token in tokens.iter(){
                println!("{}", token.regex);
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
