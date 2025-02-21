pub mod token;
use token::{
    Brace, CalcOp, Int, Literal, Operator, Parenthesis, ReservedTokenKind, Statement, Token,
    TokenKind, UnreservedTokenKind,
};

fn check_token(kind: TokenKind, str: &str, end: &mut usize, token: &mut Token) -> () {
    match kind.regex().find(str.as_bytes()) {
        Some(value) => {
            if value.end() > *end {
                // when same length of string, the faster token kind is more
                // imporatant (reserved)
                *token = Token::new(
                    kind,
                    String::from(std::str::from_utf8(value.as_bytes()).unwrap().to_string()),
                );
                *end = value.end();
            }
        }
        None => {}
    }
}

pub fn get_token(str: &str) -> Option<Token> {
    let mut end: usize = 0;
    let mut token = Token::new(TokenKind::EOF, String::from(""));

    let mut check_token_closure = |kind: TokenKind| {
        check_token(kind, str, &mut end, &mut token);
    };

    check_token_closure(TokenKind::EOF);
    check_token_closure(TokenKind::Reserved(ReservedTokenKind::WhiteSpaces));
    check_token_closure(TokenKind::Reserved(ReservedTokenKind::WhiteSpaces));
    check_token_closure(TokenKind::Reserved(ReservedTokenKind::Parenthesis(
        Parenthesis::Open,
    )));
    check_token_closure(TokenKind::Reserved(ReservedTokenKind::Parenthesis(
        Parenthesis::Close,
    )));
    check_token_closure(TokenKind::Reserved(ReservedTokenKind::Brace(Brace::Open)));
    check_token_closure(TokenKind::Reserved(ReservedTokenKind::Brace(Brace::Close)));
    check_token_closure(TokenKind::Reserved(ReservedTokenKind::Semicolon));
    check_token_closure(TokenKind::Reserved(ReservedTokenKind::Operator(
        Operator::Calc(CalcOp::Add),
    )));
    check_token_closure(TokenKind::Reserved(ReservedTokenKind::Operator(
        Operator::Calc(CalcOp::Sub),
    )));
    check_token_closure(TokenKind::Reserved(ReservedTokenKind::Operator(
        Operator::Calc(CalcOp::Mul),
    )));
    check_token_closure(TokenKind::Reserved(ReservedTokenKind::Operator(
        Operator::Calc(CalcOp::Div),
    )));
    check_token_closure(TokenKind::Reserved(ReservedTokenKind::Operator(
        Operator::Calc(CalcOp::Mod),
    )));
    check_token_closure(TokenKind::Reserved(ReservedTokenKind::Statement(
        Statement::If,
    )));
    check_token_closure(TokenKind::Reserved(ReservedTokenKind::Statement(
        Statement::While,
    )));
    check_token_closure(TokenKind::Unreserved(UnreservedTokenKind::Identifier));
    check_token_closure(TokenKind::Unreserved(UnreservedTokenKind::Literal(
        Literal::Int(Int::Dec),
    )));
    check_token_closure(TokenKind::Unreserved(UnreservedTokenKind::Literal(
        Literal::Int(Int::Hex),
    )));

    // check_token(TokenKind::EOF, str, &mut end, &mut token);
    // check_token(
    //     TokenKind::Reserved(ReservedTokenKind::WhiteSpaces),
    //     str,
    //     &mut end,
    //     &mut token,
    // );
    // check_token(
    //     TokenKind::Reserved(ReservedTokenKind::Parenthesis(Parenthesis::Open)),
    //     str,
    //     &mut end,
    //     &mut token,
    // );
    // check_token(
    //     TokenKind::Reserved(ReservedTokenKind::Parenthesis(Parenthesis::Close)),
    //     str,
    //     &mut end,
    //     &mut token,
    // );
    // check_token(
    //     TokenKind::Reserved(ReservedTokenKind::Brace(Brace::Open)),
    //     str,
    //     &mut end,
    //     &mut token,
    // );
    // check_token(
    //     TokenKind::Reserved(ReservedTokenKind::Brace(Brace::Close)),
    //     str,
    //     &mut end,
    //     &mut token,
    // );
    // check_token(
    //     TokenKind::Reserved(ReservedTokenKind::Operator(Operator::Calc(CalcOp::Add))),
    //     str,
    //     &mut end,
    //     &mut token,
    // );
    // check_token(
    //     TokenKind::Reserved(ReservedTokenKind::Operator(Operator::Calc(CalcOp::Sub))),
    //     str,
    //     &mut end,
    //     &mut token,
    // );
    // check_token(
    //     TokenKind::Reserved(ReservedTokenKind::Operator(Operator::Calc(CalcOp::Mul))),
    //     str,
    //     &mut end,
    //     &mut token,
    // );
    // check_token(
    //     TokenKind::Reserved(ReservedTokenKind::Operator(Operator::Calc(CalcOp::Div))),
    //     str,
    //     &mut end,
    //     &mut token,
    // );
    // check_token(
    //     TokenKind::Reserved(ReservedTokenKind::Operator(Operator::Calc(CalcOp::Mod))),
    //     str,
    //     &mut end,
    //     &mut token,
    // );
    // check_token(
    //     TokenKind::Reserved(ReservedTokenKind::Statement(Statement::If)),
    //     str,
    //     &mut end,
    //     &mut token,
    // );
    // check_token(
    //     TokenKind::Reserved(ReservedTokenKind::Statement(Statement::While)),
    //     str,
    //     &mut end,
    //     &mut token,
    // );
    // check_token(
    //     TokenKind::Unreserved(UnreservedTokenKind::Identifier),
    //     str,
    //     &mut end,
    //     &mut token,
    // );
    // check_token(
    //     TokenKind::Unreserved(UnreservedTokenKind::Literal(Literal::Int(Int::Dec))),
    //     str,
    //     &mut end,
    //     &mut token,
    // );
    // check_token(
    //     TokenKind::Unreserved(UnreservedTokenKind::Literal(Literal::Int(Int::Hex))),
    //     str,
    //     &mut end,
    //     &mut token,
    // );

    Option::from(token)
}

pub fn tokenize(src: &mut str) -> Vec<Token> {
    let mut tokens: std::vec::Vec<Token> = Vec::new();
    let mut remain: &mut str = src;

    loop {
        let token = get_token(remain);
        match token {
            Some(token) => match token.kind {
                TokenKind::EOF => {
                    println!("[EOF]");
                    return tokens;
                }
                _ => {
                    println!("[{:?}]", token);
                    remain = &mut remain[token.string.len()..];
                    tokens.push(token);
                }
            },
            None => {
                println!("invalid token detected");
            }
        }
    }
}
