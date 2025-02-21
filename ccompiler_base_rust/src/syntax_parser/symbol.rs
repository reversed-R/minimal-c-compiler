use std::vec;

use crate::tokenizer::token::{
    Brace, CalcOp, Int, Literal, Operator, Parenthesis, ReservedTokenKind, Statement, Token,
    TokenKind, UnreservedTokenKind,
};

pub trait SymbolTrait {
    fn all() -> Self;
}

pub trait NonterminalTrait {
    fn symbols(&self) -> Vec<Symbol>;
}

pub trait TerminalTrait {
    fn contains(&self, kind: &Self) -> bool;
}

#[derive(Debug)]
pub enum Symbol {
    Terminal(TokenKind),
    Nonterminal(Nonterminal),
}

#[derive(Debug)]
pub enum Nonterminal {
    All,
    Expr(Expr),
    Stat(Stat),
}
impl NonterminalTrait for Nonterminal {
    fn symbols(&self) -> Vec<Symbol> {
        match self {
            Self::All => vec![],
            Nonterminal::Expr(expr) => expr.symbols(),
            Nonterminal::Stat(stat) => stat.symbols(),
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    All,
    Calc,
}
impl NonterminalTrait for Expr {
    fn symbols(&self) -> Vec<Symbol> {
        match self {
            Expr::Calc => vec![Symbol::Terminal(TokenKind::Unreserved(
                UnreservedTokenKind::All,
            ))],
            Self::All => vec![],
        }
    }
}

#[derive(Debug)]
pub enum Stat {
    All,
    ControlStat(ControlStat),
    ExprStat,
}
impl NonterminalTrait for Stat {
    fn symbols(&self) -> Vec<Symbol> {
        match self {
            Stat::ExprStat => vec![Symbol::Nonterminal(Nonterminal::Expr(Expr::All))],
            Stat::ControlStat(control_stat) => control_stat.symbols(),
            Stat::All => vec![],
        }
    }
}

#[derive(Debug)]
pub enum ControlStat {
    All,
    If,
    While,
}
impl NonterminalTrait for ControlStat {
    fn symbols(&self) -> Vec<Symbol> {
        match self {
            ControlStat::If => {
                vec![
                    Symbol::Terminal(TokenKind::Reserved(ReservedTokenKind::Statement(
                        Statement::If,
                    ))),
                    Symbol::Terminal(TokenKind::Reserved(ReservedTokenKind::Parenthesis(
                        Parenthesis::Open,
                    ))),
                    Symbol::Terminal(TokenKind::Reserved(ReservedTokenKind::Parenthesis(
                        Parenthesis::Close,
                    ))),
                    Symbol::Terminal(TokenKind::Reserved(ReservedTokenKind::Brace(Brace::Open))),
                    Symbol::Terminal(TokenKind::Reserved(ReservedTokenKind::Brace(Brace::Close))),
                ]
            }
            ControlStat::While => {
                vec![
                    Symbol::Terminal(TokenKind::Reserved(ReservedTokenKind::Statement(
                        Statement::While,
                    ))),
                    Symbol::Terminal(TokenKind::Reserved(ReservedTokenKind::Parenthesis(
                        Parenthesis::Open,
                    ))),
                    Symbol::Terminal(TokenKind::Reserved(ReservedTokenKind::Parenthesis(
                        Parenthesis::Close,
                    ))),
                    Symbol::Terminal(TokenKind::Reserved(ReservedTokenKind::Brace(Brace::Open))),
                    Symbol::Terminal(TokenKind::Reserved(ReservedTokenKind::Brace(Brace::Close))),
                ]
            }
            ControlStat::All => vec![],
        }
    }
}

// impl Nonterminal {
//     pub fn symbols(&self) -> Vec<Symbol> {
//         match self {
//             Nonterminal::Expr(expr) => match expr {
//                 Expr::Calc => {
//                     vec![Symbol::Terminal(TokenKind::Unreserved(
//                         UnreservedTokenKind::All,
//                     ))]
//                 }
//                 Expr::All => vec![],
//             },
//             Nonterminal::Stat(stat) => match stat {
//                 Stat::ExprStat => vec![Symbol::Terminal(TokenKind::Unreserved(
//                     UnreservedTokenKind::All,
//                 ))],
//
//                 Stat::ControlStat(control) => match control {
//                     ControlStat::If => {
//                         vec![Symbol::Terminal(TokenKind::Reserved(
//                             ReservedTokenKind::Statement(Statement::If),
//                         ))]
//                     }
//                     ControlStat::While => {
//                         vec![Symbol::Terminal(TokenKind::Reserved(
//                             ReservedTokenKind::Statement(Statement::While),
//                         ))]
//                     }
//                     ControlStat::All => vec![],
//                 },
//                 Stat::All => vec![],
//             },
//             Nonterminal::All => vec![],
//         }
//         // vec![Symbol::Nonterminal(Nonterminal::Expr(Expr::Calc))]
//     }
// }
