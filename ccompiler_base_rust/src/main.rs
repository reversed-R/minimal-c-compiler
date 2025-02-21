mod syntax_parser;
mod tokenizer;

fn main() {
    let mut str = String::from("if(){}");
    // let mut str = String::from("if  /fnwo0x89 0x7a; (if0xd5);32 *% ifn**fewo; fiwejo if(){}  fjei");
    let tokens = tokenizer::tokenize(&mut str);
    // let last_token = tokens.remove(tokens.len() - 1);
    // println!("last_token: {:?}", last_token);
    if syntax_parser::parse(tokens) {
        println!("parse succeeded!");
    } else {
        println!("parse failed!");
    }
}
