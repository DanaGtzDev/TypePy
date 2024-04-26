use regex::Regex;

struct TokenizerStruct<'a> {
    pub regex: &'a str,
    pub tags: Vec<&'a str>
}

pub fn variable_regex(line: &String) {
    let re = Regex::new(r"(?<IDENTIFIER>[a-z_0-9]+)\s*(?<PUNCTUATOR_DOTDOT>:)\s*(?<TYPE>[a-z]+)\s*(?<PUNCTUATOR_EQUAL>=)\s*(?<LITERAL>[0-9A-Za-z'.\-]+)\s*").unwrap();
    let Some(caps) = re.captures(line) else {
        println!("INVALID SYNTAX");
        return;
    };
    println!("{}",&caps["IDENTIFIER"])
}
