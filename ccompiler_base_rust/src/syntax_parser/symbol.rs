use crate::tokenizer::token::Terminal as Term;
use std::vec;

#[derive(Debug)]
pub enum Syms {
    Natural(Vec<Syms>),
    Multi(Vec<Syms>),
    One(Vec<Syms>),
    Option(Vec<Syms>),
    Sym(Sym),
    Choices(Vec<Syms>),
}

#[derive(Debug)]
pub enum Sym {
    Term(Term),
    Nonterm(Nonterm),
}

#[derive(Debug)]
pub enum Nonterm {
    Expr,
    Stat,
    IfStat,
    WhileStat,
    ExprStat,
}

impl Nonterm {
    pub fn syms(&self) -> Syms {
        match self {
            Self::Expr => Syms::Choices(vec![Syms::Sym(Sym::Term(Term::Identifier))]),
            Self::Stat => Syms::Choices(vec![Nonterm::IfStat.syms(), Nonterm::WhileStat.syms()]),
            Self::IfStat => Syms::One(vec![
                Syms::Sym(Sym::Term(Term::If)),
                Syms::Sym(Sym::Term(Term::PareOpen)),
                Syms::Sym(Sym::Term(Term::PareClose)),
                Syms::Sym(Sym::Term(Term::BraceOpen)),
                Syms::Sym(Sym::Term(Term::BraceClose)),
            ]),
            Self::WhileStat => Syms::One(vec![
                Syms::Sym(Sym::Term(Term::While)),
                Syms::Sym(Sym::Term(Term::PareOpen)),
                Syms::Sym(Sym::Term(Term::PareClose)),
                Syms::Sym(Sym::Term(Term::BraceOpen)),
                Syms::Sym(Sym::Term(Term::BraceClose)),
            ]),
            Self::ExprStat => Syms::One(vec![
                Nonterm::Expr.syms(),
                Syms::Sym(Sym::Term(Term::Semicolon)),
            ]),
        }
    }
}
