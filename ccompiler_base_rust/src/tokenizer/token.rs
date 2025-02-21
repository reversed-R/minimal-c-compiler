use crate::syntax_parser::symbol::{SymbolTrait, TerminalTrait};
use regex::bytes::Regex;

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
    fn all(&self) -> Self {
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
    Operator(Operator),
    Statement(Statement),
}
impl SymbolTrait for ReservedTokenKind {
    fn all(&self) -> Self {
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
        }
    }
}

#[derive(Debug)]
pub enum Operator {
    All,
    Calc(CalcOp),
}
impl SymbolTrait for Operator {
    fn all(&self) -> Self {
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
    fn all(&self) -> Self {
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
    fn all(&self) -> Self {
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
    fn all(&self) -> Self {
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
    fn all(&self) -> Self {
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
    fn all(&self) -> Self {
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
