use getset::Getters;
// use crate::syntax_parser::symbol::{SymbolTrait, TerminalTrait};
use regex::bytes::Regex;

#[derive(Debug, Getters, PartialEq, Clone)]
pub struct Token {
    #[getset(get = "pub")]
    term: Terminal,
    #[getset(get = "pub")]
    value: String,
}

impl Token {
    pub fn new(term: Terminal, value: String) -> Self {
        Self { term, value }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Terminal {
    PareOpen,      // "("
    PareClose,     // ")"
    BraceOpen,     // "{"
    BraceClose,    // "}"
    BracketOpen,   // "["
    BracketClose,  // "]"
    Plus,          // "+"
    Minus,         // "-"
    Asterisk,      // "*"
    Slash,         // "/"
    Percent,       // "%"
    Equal,         // "="
    DoubleEqual,   // "=="
    NotEqual,      // "!="
    Less,          // "<"
    More,          // ">"
    EqLess,        // "<="
    EqMore,        // ">="
    Semicolon,     // ";"
    Colon,         // ":"
    WhiteSpaces,   // " " (space), "\t" (tab), "\r" (carriage return) or "\n" (line feed)
    EOF,           //
    Int,           //
    Float,         //
    Double,        //
    Char,          //
    If,            //
    While,         //
    IntLiteral,    //
    IntHexLiteral, //
    FloatLiteral,  //
    CharLiteral,   //
    StringLiteral, //
    Identifier,    //
}

impl Terminal {
    pub fn regex(&self) -> Regex {
        match self {
            Self::PareOpen => Regex::new(r"^\(").unwrap(),
            Self::PareClose => Regex::new(r"^\)").unwrap(),
            Self::BraceOpen => Regex::new(r"^\{").unwrap(),
            Self::BraceClose => Regex::new(r"^\}").unwrap(),
            Self::BracketOpen => Regex::new(r"^\[").unwrap(),
            Self::BracketClose => Regex::new(r"^\]").unwrap(),
            Self::Plus => Regex::new(r"^\+").unwrap(),
            Self::Minus => Regex::new(r"^\-").unwrap(),
            Self::Asterisk => Regex::new(r"^\*").unwrap(),
            Self::Slash => Regex::new(r"^\/").unwrap(),
            Self::Percent => Regex::new(r"^\%").unwrap(),
            Self::Equal => Regex::new(r"^=").unwrap(),
            Self::DoubleEqual => Regex::new(r"^==").unwrap(),
            Self::NotEqual => Regex::new(r"^!=").unwrap(),
            Self::Less => Regex::new(r"^<").unwrap(),
            Self::More => Regex::new(r"^>").unwrap(),
            Self::EqLess => Regex::new(r"^<=").unwrap(),
            Self::EqMore => Regex::new(r"^>=").unwrap(),
            Self::Semicolon => Regex::new(r"^;").unwrap(),
            Self::Colon => Regex::new(r"^:").unwrap(),
            Self::WhiteSpaces => Regex::new(r"^\s+").unwrap(),
            Self::Int => Regex::new(r"^int").unwrap(),
            Self::Float => Regex::new(r"^float").unwrap(),
            Self::Double => Regex::new(r"^double").unwrap(),
            Self::Char => Regex::new(r"^char").unwrap(),
            Self::If => Regex::new(r"^if").unwrap(),
            Self::While => Regex::new(r"^while").unwrap(),
            Self::IntLiteral => Regex::new(r"^0|^([1-9][0-9]*)").unwrap(),
            Self::IntHexLiteral => Regex::new(r"^0x[0-9a-fA-F]+").unwrap(),
            Self::FloatLiteral => Regex::new(r"^(0|[1-9][0-9]*)\.[0-9]+").unwrap(),
            Self::CharLiteral => Regex::new(r"^'.'").unwrap(),
            Self::StringLiteral => Regex::new("^\"\\w*\"").unwrap(),
            Self::Identifier => Regex::new(r"^[a-zA-Z][0-9a-zA-Z]*").unwrap(),
            Self::EOF => Regex::new(r"^$").unwrap(),
        }
    }
}
