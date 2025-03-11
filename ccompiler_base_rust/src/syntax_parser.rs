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

pub struct Node<'a> {
    sym: &'a Sym,
    terms: &'a Vec<Term>,
}

pub fn match_syms_slice(syms_slice: &Slice<Syms>, terms: &Slice<Term>) -> bool {
    false
}

pub fn match_syms(syms: &Syms, terms: &Slice<Term>) -> bool {
    match syms {
        Syms::Natural(vec) => {
            let mut count = 0;
            loop {
                if !match_syms_slice(&Slice::new(vec, 0, EndIndex::ToEnd), terms) {
                    break;
                }
                count += 1;
            }

            count > 0
        }
        Syms::Multi(vec) => {
            let mut count = 0;
            loop {
                if !match_syms_slice(&Slice::new(vec, 0, EndIndex::ToEnd), terms) {
                    break;
                }
                count += 1;
            }

            true
        }
        Syms::One(vec) => match_syms_slice(&Slice::new(vec, 0, EndIndex::ToEnd), terms),
        Syms::Option(vec) => {
            match_syms_slice(&Slice::new(vec, 0, EndIndex::ToEnd), terms);
            true
        }
        Syms::Sym(sym) => match sym {
            Sym::Term(term) => {
                if let Some(terms_at_start) = terms.vec().get(terms.range().start) {
                    term == terms_at_start
                } else {
                    false
                }
            }
            Sym::Nonterm(nonterm) => match_syms(&nonterm.syms(), terms),
        },
        Syms::Choices(choices) => {
            let mut index = 0;
            let mut end: usize = 0;
            // 最長一致
            loop {
                match choices.get(index) {
                    Some(syms) => {
                        match_syms(&syms, terms);
                    }
                    None => break,
                }
                index += 1;
            }
            false
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
