use regex::bytes::Regex;

#[derive(Debug)]
pub enum TokenKind {
    Reserved(ReservedTokenKind),
    Unreseved(UnreservedTokenKind),
    EOF,
}

#[derive(Debug)]
pub enum ReservedTokenKind {
    WhiteSpaces,
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
    pub fn regex(&self) -> regex::bytes::Regex {
        match &self {
            TokenKind::Reserved(reserved) => match reserved {
                ReservedTokenKind::WhiteSpaces => Regex::new(r"^[ ]+").unwrap(),
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
    pub kind: TokenKind,
    pub string: String,
}

impl Token {
    pub fn new(kind: TokenKind, string: String) -> Token {
        Token { kind, string }
    }
}
