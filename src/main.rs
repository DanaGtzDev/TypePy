use std::fs::read_to_string;
//New branch
fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
   //"dummy/main.ty"
   let lines = read_lines("dummy/main.ty");

   for line in lines.iter(){
    println!("{}", line);
   }
}
