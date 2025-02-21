use crate::tokenizer::token::{
    CalcOp, Int, Literal, Operator, ReservedTokenKind, Statement, Token, TokenKind,
    UnreservedTokenKind,
};

pub trait SymbolTrait {
    fn all(&self) -> Self;
}

pub trait NonterminalTrait {
    fn symbols(&self) -> Vec<Symbol>;
}

pub trait TerminalTrait {
    fn contains(&self, kind: &Self) -> bool;
}

pub enum Symbol {
    Terminal(TokenKind),
    Nonterminal(Nonterminal),
}

pub enum Nonterminal {
    Expr(Expr),
    Stat(Stat),
}

pub enum Expr {
    Calc,
}

pub enum Stat {
    ControlStat(ControlStat),
    ExprStat(ExprStat),
}

pub enum ExprStat {
    Identifier,
    Literal,
}

pub enum ControlStat {
    If,
    While,
}

impl Nonterminal {
    pub fn symbols(&self) -> Vec<Symbol> {
        match self {
            Nonterminal::Expr(expr) => match expr {
                Expr::Calc => {
                    vec![Symbol::Terminal(TokenKind::Unreserved(
                        UnreservedTokenKind::Identifier,
                    ))]
                }
            },
            Nonterminal::Stat(stat) => match stat {
                Stat::ExprStat(expr) => match expr {
                    ExprStat::Identifier => {
                        vec![Symbol::Terminal(TokenKind::Unreserved(
                            UnreservedTokenKind::Identifier,
                        ))]
                    }
                    ExprStat::Literal => {
                        vec![Symbol::Terminal(TokenKind::Unreserved(
                            UnreservedTokenKind::Literal(Literal::All),
                        ))]
                    }
                },
                Stat::ControlStat(control) => match control {
                    ControlStat::If => {
                        vec![Symbol::Terminal(TokenKind::Reserved(
                            ReservedTokenKind::Statement(Statement::If),
                        ))]
                    }
                    ControlStat::While => {
                        vec![Symbol::Terminal(TokenKind::Reserved(
                            ReservedTokenKind::Statement(Statement::While),
                        ))]
                    }
                },
            },
        }
        // vec![Symbol::Nonterminal(Nonterminal::Expr(Expr::Calc))]
    }
}
