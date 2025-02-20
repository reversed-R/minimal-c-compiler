mod tokenizer;

fn main() {
    tokenizer::match_token("");
    println!("----");
    tokenizer::match_token("f");
    println!("----");
    tokenizer::match_token("if");
    println!("----");
    tokenizer::match_token("if ");
    println!("----");
    tokenizer::match_token("if sfad");
    println!("----");
    tokenizer::match_token("ifssfad");
    println!("----");
}
