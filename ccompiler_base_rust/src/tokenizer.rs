mod token;
// use regex::bytes::Match;
use token::{ReservedTokenKind, Statement, Token, TokenKind, UnreservedTokenKind};

// fn match_regex(regex: regex::bytes::Regex, str: &str) -> Option<Match> {
//     regex.find(str.as_bytes())
// }

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

    check_token(TokenKind::EOF, str, &mut end, &mut token);
    check_token(
        TokenKind::Reserved(ReservedTokenKind::WhiteSpaces),
        str,
        &mut end,
        &mut token,
    );
    check_token(
        TokenKind::Reserved(ReservedTokenKind::Statement(Statement::If)),
        str,
        &mut end,
        &mut token,
    );
    check_token(
        TokenKind::Reserved(ReservedTokenKind::Statement(Statement::While)),
        str,
        &mut end,
        &mut token,
    );
    check_token(
        TokenKind::Unreseved(UnreservedTokenKind::Identifier),
        str,
        &mut end,
        &mut token,
    );

    Option::from(token)
}

pub fn tokenize(src: &mut str) -> () {
    let mut tokens: std::vec::Vec<Token> = Vec::new();
    let mut remain: &mut str = src;
    // let token = get_token(remain);

    loop {
        let token = get_token(remain);
        match token {
            Some(token) => match token.kind {
                TokenKind::EOF => {
                    println!("[EOF]");
                    break;
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
    //
    // match token {
    //     Some(token) => {
    //         remain = &mut remain[token.string.len()..];
    //         while () {}
    //     }
    //     None => {}
    // }
    // match token {
    //     Some(token) => {
    //         remain = &mut remain[token.string.len()..];
    //     }
    //     None => {}
    // }
}
