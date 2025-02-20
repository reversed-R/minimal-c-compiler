use regex::bytes::{Match, Regex};
use std::cmp::max;

// pub enum TokenKind {
//     // CalcOp
//     Add, // "+"
//     Sub, // "-"
//     Mul, // "*"
//     Div, // "/"
//     Mod, // "%"
//     // CondOp
//     Equal,       // "=="
//     NotEqual,    // "!="
//     Larger,      // ">"
//     Smaller,     // "<"
//     EqOrLarger,  // ">="
//     EqOrSmaller, // "<="
//     // AssignOp
//     Assign,    // "="
//     AddAssign, // "+="
//     SubAssign, // "-="
//     MulAssign, // "*="
//     DivAssign, // "/="
//     ModAssign, // "%="
// }

// pub enum CalcOpTokenKind {}
// pub enum CondOpTokenKind {}
// pub enum AssignOpTokenKind {}

// pub enum TokenType {
//     CalcOp,
//     CondOp,
//     AssignOp,
//     ReservedType,
//     Literal,
//     EOF,
// }
//
// pub struct ReservedToken {
//     token_type: TokenType,
//     name: TokenKind,
//     regex: Regex, // token regular expression string
// }
//
// impl ReservedToken {
//     fn new(token_type: TokenType, name: TokenKind, regex: Regex) -> ReservedToken {
//         ReservedToken {
//             token_type,
//             name,
//             regex,
//         }
//     }
// }
//
// enum TokenReservation {
//     Reserved(TokenKind),
//     NotReserved(String),
// }
//
// pub struct Token {
//     token_type: TokenType,
//     name: TokenKind,
//     reservation: TokenReservation,
// }

// pub fn tokenize(src: String) -> () {
//     let mut remain = src;
//     loop {
//         let token = get_token(&mut remain);
//
//         match token.token_type {
//             TokenType::EOF => {
//                 break;
//             }
//             _ => {
//                 continue;
//             }
//         }
//     }
// }

#[derive(Debug)]
pub enum TokenKind {
    Reserved(ReservedTokenKind),
    Unreseved(UnreservedTokenKind),
    EOF,
}

#[derive(Debug)]
pub enum ReservedTokenKind {
    Operator(Operator),
    Statement(Statement),
}

#[derive(Debug)]
pub enum Operator {
    Calc(CalcOp),
}

#[derive(Debug)]
pub enum CalcOp {
    Add, // "+"
    Sub, // "-"
    Mul, // "*"
    Div, // "/"
    Mod, // "%"
}

#[derive(Debug)]
pub enum Statement {
    If,
    While,
}

#[derive(Debug)]
pub enum UnreservedTokenKind {
    Literal(Literal),
    Identifier,
}

#[derive(Debug)]
pub enum Literal {
    Int(Int),
}

#[derive(Debug)]
pub enum Int {
    Dec,
    Hex,
}

impl TokenKind {
    fn regex(&self) -> regex::bytes::Regex {
        match &self {
            TokenKind::Reserved(reserved) => match reserved {
                ReservedTokenKind::Operator(operator) => match operator {
                    Operator::Calc(calc_operator) => match calc_operator {
                        CalcOp::Add => Regex::new(r"^\+").unwrap(),
                        CalcOp::Sub => Regex::new(r"^\-").unwrap(),
                        CalcOp::Mul => Regex::new(r"^\*").unwrap(),
                        CalcOp::Div => Regex::new(r"^\/").unwrap(),
                        CalcOp::Mod => Regex::new(r"^\%").unwrap(),
                    },
                },
                ReservedTokenKind::Statement(statement) => match statement {
                    Statement::If => Regex::new(r"^if").unwrap(),
                    Statement::While => Regex::new(r"^while").unwrap(),
                },
            },
            TokenKind::Unreseved(unreserved) => match unreserved {
                UnreservedTokenKind::Literal(literal) => match literal {
                    Literal::Int(int) => match int {
                        Int::Dec => Regex::new(r"^0|([1-9][0-9]*)").unwrap(),
                        Int::Hex => Regex::new(r"^0x[0-9a-fA-F]+").unwrap(),
                    },
                },
                UnreservedTokenKind::Identifier => Regex::new(r"^[a-zA-Z][0-9a-zA-Z]*").unwrap(),
            },
            TokenKind::EOF => Regex::new(r"^$").unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    string: String,
}

impl Token {
    fn new(kind: TokenKind, string: String) -> Token {
        Token { kind, string }
    }
}

fn match_regex(regex: regex::bytes::Regex, str: &str) -> Option<Match> {
    regex.find(str.as_bytes())
}

fn check_token(kind: TokenKind, str: &str, end: &mut usize, token: &mut Token) -> () {
    match match_regex(kind.regex(), str) {
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
            println!("Some: {:?}", value)
        }
        None => {
            println!("None")
        }
    }
}

pub fn match_token(str: &str) -> () {
    let mut end: usize = 0;
    let mut token = Token::new(TokenKind::EOF, String::from(""));

    // TokenKind::EOF
    check_token(TokenKind::EOF, str, &mut end, &mut token);
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

    println!("{:?}", token);
    // match match_regex(TokenKind::EOF.regex(), str) {
    //     Some(value) => {
    //         end = max(value.end(), end);
    //         if value.end() > end {
    //             token = Token::new(TokenKind::EOF, String::from(value.as_bytes()));
    //         }
    //         println!("Some: {:?}", value)
    //     }
    //     None => {
    //         println!("None")
    //     }
    // }
    // TokenKind::ReservedTokenKind
    // match match_regex(
    //     TokenKind::Reserved(ReservedTokenKind::Statement(Statement::If)).regex(),
    //     str,
    // ) {
    //     Some(value) => {
    //         end = max(value.end(), end);
    //         println!("Some: {:?}", value)
    //     }
    //     None => {
    //         println!("None")
    //     }
    // }
    // match match_regex(
    //     TokenKind::Unreseved(UnreservedTokenKind::Identifier).regex(),
    //     str,
    // ) {
    //     Some(value) => {
    //         end = max(value.end(), end);
    //         println!("Some: {:?}", value)
    //     }
    //     None => {
    //         println!("None")
    //     }
    // }
}

// fn get_token(remain: &mut str) -> ReservedToken {
//     // let tokens = [ReservedToken::new(
//     //     TokenType::CalcOp,
//     //     TokenKind::Add,
//     //     Regex::new(r"\+").unwrap(),
//     // )];
//
//     ReservedToken::new(token_type, name, str)
// }
