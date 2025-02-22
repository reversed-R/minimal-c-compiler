use crate::syntax_parser::symbol::{SymbolTrait, TerminalTrait};
use regex::bytes::Regex;

#[derive(Debug, PartialEq)]
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

// pub trait TokenKindTrait {
//     fn regex(&self) -> Regex;
// }

// TokenKind and their children enum must has All to express their all children
#[derive(Debug)]
pub enum TokenKind {
    All,
    Reserved(ReservedTokenKind),
    Unreserved(UnreservedTokenKind),
    EOF,
}

impl SymbolTrait for TokenKind {
    fn all() -> Self {
        TokenKind::All
    }
}
impl TerminalTrait for TokenKind {
    fn contains(&self, kind: &TokenKind) -> bool {
        match self {
            TokenKind::All => true,
            TokenKind::Reserved(reserved) => match kind {
                TokenKind::Reserved(reserved_kind) => reserved.contains(reserved_kind),
                _ => false,
            },
            TokenKind::Unreserved(unreserved) => match kind {
                TokenKind::Unreserved(unreserved_kind) => unreserved.contains(unreserved_kind),
                _ => false,
            },

            TokenKind::EOF => false,
        }
    }
}

#[derive(Debug)]
pub enum ReservedTokenKind {
    All,
    WhiteSpaces,
    Parenthesis(Parenthesis),
    Brace(Brace),
    Semicolon,
    Operator(Operator),
    Statement(Statement),
}
impl SymbolTrait for ReservedTokenKind {
    fn all() -> Self {
        ReservedTokenKind::All
    }
}
impl TerminalTrait for ReservedTokenKind {
    fn contains(&self, kind: &ReservedTokenKind) -> bool {
        match self {
            ReservedTokenKind::All => true,
            ReservedTokenKind::Statement(statement) => match kind {
                ReservedTokenKind::Statement(stat_kind) => statement.contains(stat_kind),
                _ => false,
            },
            ReservedTokenKind::Operator(operator) => match kind {
                ReservedTokenKind::Operator(operator_kind) => operator.contains(operator_kind),
                _ => false,
            },
            ReservedTokenKind::WhiteSpaces => match kind {
                ReservedTokenKind::WhiteSpaces => true,
                _ => false,
            },
            ReservedTokenKind::Parenthesis(pare) => match kind {
                ReservedTokenKind::Parenthesis(pare_kind) => pare.contains(pare_kind),
                _ => false,
            },
            ReservedTokenKind::Brace(brace) => match kind {
                ReservedTokenKind::Brace(brace_kind) => brace.contains(brace_kind),
                _ => false,
            },
            ReservedTokenKind::Semicolon => match kind {
                ReservedTokenKind::Semicolon => true,
                _ => false,
            },
        }
    }
}

#[derive(Debug)]
pub enum Parenthesis {
    All,
    Open,
    Close,
}
impl TerminalTrait for Parenthesis {
    fn contains(&self, kind: &Self) -> bool {
        match self {
            Self::All => true,
            Self::Open => match kind {
                Self::Open => true,
                _ => false,
            },
            Self::Close => match kind {
                Self::Close => true,
                _ => false,
            },
        }
    }
}

#[derive(Debug)]
pub enum Brace {
    All,
    Open,
    Close,
}
impl TerminalTrait for Brace {
    fn contains(&self, kind: &Self) -> bool {
        match self {
            Self::All => true,
            Self::Open => match kind {
                Self::Open => true,
                _ => false,
            },
            Self::Close => match kind {
                Self::Close => true,
                _ => false,
            },
        }
    }
}

#[derive(Debug)]
pub enum Operator {
    All,
    Calc(CalcOp),
}
impl SymbolTrait for Operator {
    fn all() -> Self {
        Operator::All
    }
}
impl TerminalTrait for Operator {
    fn contains(&self, kind: &Operator) -> bool {
        match self {
            Operator::All => true,
            Operator::Calc(calc_op) => match kind {
                Operator::Calc(calc_kind) => calc_op.contains(calc_kind),
                _ => false,
            },
        }
    }
}

#[derive(Debug)]
pub enum CalcOp {
    All,
    Add, // "+"
    Sub, // "-"
    Mul, // "*"
    Div, // "/"
    Mod, // "%"
}
impl SymbolTrait for CalcOp {
    fn all() -> Self {
        CalcOp::All
    }
}
impl TerminalTrait for CalcOp {
    fn contains(&self, kind: &CalcOp) -> bool {
        match self {
            CalcOp::All => true,
            CalcOp::Add => match kind {
                CalcOp::Add => true,
                _ => false,
            },
            CalcOp::Sub => match kind {
                CalcOp::Sub => true,
                _ => false,
            },
            CalcOp::Mul => match kind {
                CalcOp::Mul => true,
                _ => false,
            },
            CalcOp::Div => match kind {
                CalcOp::Div => true,
                _ => false,
            },
            CalcOp::Mod => match kind {
                CalcOp::Mod => true,
                _ => false,
            },
        }
    }
}

#[derive(Debug)]
pub enum Statement {
    All,
    If,
    While,
}
impl SymbolTrait for Statement {
    fn all() -> Self {
        Statement::All
    }
}
impl TerminalTrait for Statement {
    fn contains(&self, kind: &Statement) -> bool {
        match self {
            Statement::All => true,
            Statement::If => match kind {
                Statement::If => true,
                _ => false,
            },
            Statement::While => match kind {
                Statement::While => true,
                _ => false,
            },
        }
    }
}

#[derive(Debug)]
pub enum UnreservedTokenKind {
    All,
    Literal(Literal),
    Identifier,
}
impl SymbolTrait for UnreservedTokenKind {
    fn all() -> Self {
        UnreservedTokenKind::All
    }
}
impl TerminalTrait for UnreservedTokenKind {
    fn contains(&self, kind: &UnreservedTokenKind) -> bool {
        match self {
            UnreservedTokenKind::All => true,
            UnreservedTokenKind::Identifier => match kind {
                UnreservedTokenKind::Identifier => true,
                _ => false,
            },
            UnreservedTokenKind::Literal(literal) => match kind {
                UnreservedTokenKind::Literal(literal_kind) => literal.contains(literal_kind),
                _ => false,
            },
        }
    }
}

#[derive(Debug)]
pub enum Literal {
    All,
    Int(Int),
}
impl SymbolTrait for Literal {
    fn all() -> Self {
        Literal::All
    }
}
impl TerminalTrait for Literal {
    fn contains(&self, kind: &Literal) -> bool {
        match self {
            Literal::All => true,
            Literal::Int(int) => match kind {
                Literal::Int(int_kind) => int.contains(int_kind),
                _ => false,
            },
        }
    }
}

#[derive(Debug)]
pub enum Int {
    All,
    Dec,
    Hex,
}
impl SymbolTrait for Int {
    fn all() -> Self {
        Int::All
    }
}
impl TerminalTrait for Int {
    fn contains(&self, kind: &Int) -> bool {
        match self {
            Int::All => true,
            Int::Dec => match kind {
                Int::Dec => true,
                _ => false,
            },
            Int::Hex => match kind {
                Int::Hex => true,
                _ => false,
            },
        }
    }
}

impl TokenKind {
    pub fn regex(&self) -> regex::bytes::Regex {
        match &self {
            TokenKind::Reserved(reserved) => match reserved {
                ReservedTokenKind::WhiteSpaces => Regex::new(r"^[ ]+").unwrap(),
                ReservedTokenKind::Parenthesis(pare) => match pare {
                    Parenthesis::Open => Regex::new(r"^\(").unwrap(),
                    Parenthesis::Close => Regex::new(r"^\)").unwrap(),
                    Parenthesis::All => Regex::new(r"").unwrap(),
                },
                ReservedTokenKind::Brace(brace) => match brace {
                    Brace::Open => Regex::new(r"^\{").unwrap(),
                    Brace::Close => Regex::new(r"^\}").unwrap(),
                    Brace::All => Regex::new(r"").unwrap(),
                },
                ReservedTokenKind::Semicolon => Regex::new(r"^;").unwrap(),
                ReservedTokenKind::Operator(operator) => match operator {
                    Operator::Calc(calc_operator) => match calc_operator {
                        CalcOp::Add => Regex::new(r"^\+").unwrap(),
                        CalcOp::Sub => Regex::new(r"^\-").unwrap(),
                        CalcOp::Mul => Regex::new(r"^\*").unwrap(),
                        CalcOp::Div => Regex::new(r"^\/").unwrap(),
                        CalcOp::Mod => Regex::new(r"^\%").unwrap(),
                        CalcOp::All => Regex::new(r"").unwrap(),
                    },
                    Operator::All => Regex::new(r"").unwrap(),
                },
                ReservedTokenKind::Statement(statement) => match statement {
                    Statement::If => Regex::new(r"^if").unwrap(),
                    Statement::While => Regex::new(r"^while").unwrap(),
                    Statement::All => Regex::new(r"").unwrap(),
                },
                ReservedTokenKind::All => Regex::new(r"").unwrap(),
            },
            TokenKind::Unreserved(unreserved) => match unreserved {
                UnreservedTokenKind::Literal(literal) => match literal {
                    Literal::Int(int) => match int {
                        Int::Dec => Regex::new(r"^0|^([1-9][0-9]*)").unwrap(),
                        Int::Hex => Regex::new(r"^0x[0-9a-fA-F]+").unwrap(),
                        Int::All => Regex::new(r"").unwrap(),
                    },
                    Literal::All => Regex::new(r"").unwrap(),
                },
                UnreservedTokenKind::Identifier => Regex::new(r"^[a-zA-Z][0-9a-zA-Z]*").unwrap(),
                UnreservedTokenKind::All => Regex::new(r"").unwrap(),
            },
            TokenKind::EOF => Regex::new(r"^$").unwrap(),
            TokenKind::All => Regex::new(r"").unwrap(),
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
