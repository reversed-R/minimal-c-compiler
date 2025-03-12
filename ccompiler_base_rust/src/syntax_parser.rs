use crate::shared::slice::{EndIndex, Slice};
use crate::tokenizer::token::{Terminal as Term, Token};
use symbol::{Nonterm, Sym, Syms};

pub mod symbol;

pub fn parse(tokens: Vec<Token>) -> bool {
    let mut result = false;
    // println!("parse()---->");
    // if match_symbols(&Expr::Calc.symbols(), 0, &tokens, 0) {
    //     println!("Expr::Calc");
    //     result = true;
    // }
    // println!("------");
    // if match_symbols(&Stat::ExprStat.symbols(), 0, &tokens, 0) {
    //     println!("Stat::ExprStat");
    //     result = true;
    // }
    // println!("------");
    // if match_symbols(&Stat::ControlStat(ControlStat::If).symbols(), 0, &tokens, 0) {
    //     println!("ControlStat::If");
    //     result = true;
    // }
    // println!("------");

    result
}

pub struct SymMatch {
    sym: Sym,
    tokens: Slice<Token>,
}

impl SymMatch {
    pub fn new(sym: Sym, tokens: Slice<Token>) -> Self {
        Self { sym, tokens }
    }
}

pub fn match_syms_slice<'a>(syms_slice: Slice<Syms>, tokens: Slice<Token>) -> Option<SymMatch> {
    let mut index = syms_slice.range().start;
    let mut result: Option<SymMatch> = None;

    while index < syms_slice.range().end {
        if let Some(syms) = syms_slice.vec().get(index) {
            match match_syms(syms.clone(), tokens.clone()) {
                Some(matched) => {
                    result = Some(matched);
                }
                None => {}
            }
        }
        index += 1;
    }

    result
}

pub fn match_syms<'a>(syms: Syms, tokens: Slice<Token>) -> Option<SymMatch> {
    match syms {
        Syms::Natural(vec) => {
            let mut count = 0;
            loop {
                match match_syms_slice(Slice::new(vec.clone(), 0, EndIndex::ToEnd), tokens.clone())
                {
                    Some(_) => {}
                    None => break,
                }
                count += 1;
            }

            None
        }
        Syms::Multi(vec) => {
            let mut count = 0;
            loop {
                match match_syms_slice(Slice::new(vec.clone(), 0, EndIndex::ToEnd), tokens.clone())
                {
                    Some(_) => {}
                    None => break,
                }
                count += 1;
            }

            None
        }
        Syms::One(vec) => match_syms_slice(Slice::new(vec, 0, EndIndex::ToEnd), tokens),
        Syms::Option(vec) => {
            match_syms_slice(Slice::new(vec, 0, EndIndex::ToEnd), tokens);
            None
        }
        Syms::Sym(sym) => match sym {
            Sym::Term(term) => {
                if let Some(token) = tokens.vec().get(tokens.range().start) {
                    if &term == token.term() {
                        Some(SymMatch::new(
                            sym.clone(),
                            Slice::new(
                                tokens.vec().clone(),
                                tokens.range().start,
                                EndIndex::I(tokens.range().start + 1),
                            ),
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Sym::Nonterm(nonterm) => {
                if let Some(value) = match_syms(nonterm.syms(), tokens.clone()) {
                    Some(value)
                } else {
                    None
                }
            }
        },
        Syms::Choices(choices) => {
            let mut index = 0;
            let mut end: usize = 0;
            // 最長一致
            loop {
                match choices.get(index) {
                    Some(syms) => {
                        match_syms(syms.clone(), tokens.clone());
                    }
                    None => break,
                }
                index += 1;
            }
            None
        }
    }
}
