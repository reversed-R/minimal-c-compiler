pub mod token;
use token::{Terminal, Token};

fn match_terminal<'a>(term: &Terminal, str: &'a str) -> Option<&'a str> {
    match term.regex().find(str.as_bytes()) {
        Some(value) => Some(&str[..value.end()]),
        None => None,
    }
}

fn get_token(str: &str) -> Option<Token> {
    let mut end: usize = 0;
    let mut token: Option<Token> = None;
    let mut match_terminal_closure = |term: Terminal| match match_terminal(&term, str) {
        Some(value) => match token {
            Some(_) => {
                if end < value.len() {
                    end = value.len();
                    token = Some(Token::new(term, value.to_string()));
                }
            }
            None => {
                end = value.len();
                token = Some(Token::new(term, value.to_string()));
            }
        },
        None => {}
    };

    // 最長一致
    // 短い終端記号を先にチェックする
    // そうでないと一致しうる終端記号(IdentifierやLiteral)に奪われる
    match_terminal_closure(Terminal::EOF);
    match_terminal_closure(Terminal::Plus);
    match_terminal_closure(Terminal::Minus);
    match_terminal_closure(Terminal::Semicolon);
    match_terminal_closure(Terminal::PareOpen);
    match_terminal_closure(Terminal::PareClose);
    match_terminal_closure(Terminal::If);
    match_terminal_closure(Terminal::WhiteSpaces);
    match_terminal_closure(Terminal::Identifier);

    token
}

pub fn tokenize(src: &str) -> Vec<Token> {
    let mut tokens: std::vec::Vec<Token> = Vec::new();
    let mut string = src.to_string();
    let mut remain: &mut str = &mut string;

    loop {
        let token = get_token(remain);
        match token {
            Some(token) => match token.term() {
                Terminal::EOF => {
                    println!("[EOF]");
                    return tokens;
                }
                _ => {
                    println!("[{:?}]", token);
                    remain = &mut remain[token.value().len()..];
                    tokens.push(token);
                }
            },
            None => {
                println!("invalid token detected");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::token::{Terminal, Token};

    use super::tokenize;

    #[test]
    fn test1() {
        assert_eq!(
            tokenize("if sfasd"),
            vec![
                Token::new(Terminal::If, "if".to_string()),
                Token::new(Terminal::WhiteSpaces, " ".to_string()),
                Token::new(Terminal::Identifier, "sfasd".to_string()),
            ]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            tokenize("if (ifsfasd)  fds;"),
            vec![
                Token::new(Terminal::If, "if".to_string()),
                Token::new(Terminal::WhiteSpaces, " ".to_string()),
                Token::new(Terminal::PareOpen, "(".to_string()),
                Token::new(Terminal::Identifier, "ifsfasd".to_string()),
                Token::new(Terminal::PareClose, ")".to_string()),
                Token::new(Terminal::WhiteSpaces, "  ".to_string()),
                Token::new(Terminal::Identifier, "fds".to_string()),
                Token::new(Terminal::Semicolon, ";".to_string()),
            ]
        );
    }
}
