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

pub struct SymMatch<'a> {
    sym: &'a Sym,
    tokens: &'a Slice<'a, Token>,
}

impl<'a> SymMatch<'a> {
    pub fn new(sym: &'a Sym, tokens: &'a Slice<'a, Token>) -> Self {
        Self { sym, tokens }
    }
}

pub fn match_syms_slice<'a>(
    syms_slice: &'a Slice<Syms>,
    tokens: &'a Slice<Token>,
) -> Option<SymMatch<'a>> {
    let mut index = syms_slice.range().start;
    let mut result: Option<SymMatch<'a>> = None;

    while index < syms_slice.range().end {
        if let Some(syms) = syms_slice.vec().get(index) {
            match match_syms(syms, tokens) {
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

pub fn match_syms<'a>(syms: &'a Syms, tokens: &'a Slice<Token>) -> Option<SymMatch<'a>> {
    match syms {
        Syms::Natural(vec) => {
            let mut count = 0;
            loop {
                match match_syms_slice(&Slice::new(vec, 0, EndIndex::ToEnd), tokens) {
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
                match match_syms_slice(&Slice::new(vec, 0, EndIndex::ToEnd), tokens) {
                    Some(_) => {}
                    None => break,
                }
                count += 1;
            }

            None
        }
        Syms::One(vec) => match_syms_slice(&Slice::new(vec, 0, EndIndex::ToEnd), tokens),
        Syms::Option(vec) => {
            match_syms_slice(&Slice::new(vec, 0, EndIndex::ToEnd), tokens);
            None
        }
        Syms::Sym(sym) => match sym {
            Sym::Term(term) => {
                if let Some(token) = tokens.vec().get(tokens.range().start) {
                    if term == token.term() {
                        Some(SymMatch::new(
                            sym,
                            &Slice::new(
                                tokens.vec(),
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
            Sym::Nonterm(nonterm) => match_syms(&nonterm.syms(), tokens),
        },
        Syms::Choices(choices) => {
            let mut index = 0;
            let mut end: usize = 0;
            // 最長一致
            loop {
                match choices.get(index) {
                    Some(syms) => {
                        match_syms(&syms, tokens);
                    }
                    None => break,
                }
                index += 1;
            }
            None
        }
    }
}

// pub fn match_symbols(
//     symbols: &Vec<Symbol>,
//     syms_index: usize,
//     terminals: &Vec<Token>,
//     terms_index: usize,
// ) -> bool {
//     println!("match_symbols()---->");
//     match symbols.get(syms_index) {
//         Some(symbol) => match symbol {
//             Symbol::Terminal(terminal) => match terminals.get(terms_index) {
//                 Some(token) => {
//                     println!(
//                         "Symbol[{}]: {:?},\nTerminals[{}]: {:?}",
//                         syms_index, symbol, terms_index, token
//                     );
//                     if terminal.contains(&token.kind) {
//                         if symbols.len() > syms_index + 1 && terminals.len() > terms_index + 1 {
//                             match_symbols(symbols, syms_index + 1, terminals, terms_index + 1)
//                         } else if symbols.len() == syms_index + 1
//                             && terminals.len() == terms_index + 1
//                         {
//                             true
//                         } else {
//                             false
//                         }
//                     // must make symbols[1..], terminals[1..]
//                     } else {
//                         false
//                     }
//                 }
//                 None => false,
//             },
//             Symbol::Nonterminal(nonterminal) => {
//                 println!("Symbol[{}]: {:?}", syms_index, nonterminal,);
//                 match_symbols(&nonterminal.symbols(), syms_index, terminals, terms_index)
//             }
//         },
//         None => symbols.len() == syms_index && terminals.len() == terms_index,
//     }
// }
