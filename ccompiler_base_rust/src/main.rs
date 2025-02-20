mod tokenizer;

fn main() {
    // tokenizer::match_token("");
    // println!("----");
    // tokenizer::match_token("f");
    // println!("----");
    // tokenizer::match_token("if");
    // println!("----");
    // tokenizer::match_token("if ");
    // println!("----");
    // tokenizer::match_token("if sfad");
    // println!("----");
    // tokenizer::match_token("ifssfad");
    // println!("----");
    let mut str = String::from("if  /fnwo0x89 0x7a if0xd5 32 *% ifn**fewo fiwejo if  fjei");
    tokenizer::tokenize(&mut str);
}
