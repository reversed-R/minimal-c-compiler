use symbol::{ControlStat, Expr, NonterminalTrait, Stat, Symbol, TerminalTrait};

use crate::tokenizer::token::Token;

pub mod symbol;

pub fn parse(tokens: Vec<Token>) -> bool {
    let mut result = false;
    println!("parse()---->");
    if match_symbols(&Expr::Calc.symbols(), 0, &tokens, 0) {
        println!("Expr::Calc");
        result = true;
    }
    println!("------");
    if match_symbols(&Stat::ExprStat.symbols(), 0, &tokens, 0) {
        println!("Stat::ExprStat");
        result = true;
    }
    println!("------");
    if match_symbols(&Stat::ControlStat(ControlStat::If).symbols(), 0, &tokens, 0) {
        println!("ControlStat::If");
        result = true;
    }
    println!("------");

    result
}

pub fn match_symbols(
    symbols: &Vec<Symbol>,
    syms_index: usize,
    terminals: &Vec<Token>,
    terms_index: usize,
) -> bool {
    println!("match_symbols()---->");
    match symbols.get(syms_index) {
        Some(symbol) => match symbol {
            Symbol::Terminal(terminal) => match terminals.get(terms_index) {
                Some(token) => {
                    println!(
                        "Symbol[{}]: {:?},\nTerminals[{}]: {:?}",
                        syms_index, symbol, terms_index, token
                    );
                    if terminal.contains(&token.kind) {
                        if symbols.len() > syms_index + 1 && terminals.len() > terms_index + 1 {
                            match_symbols(symbols, syms_index + 1, terminals, terms_index + 1)
                        } else if symbols.len() == syms_index + 1
                            && terminals.len() == terms_index + 1
                        {
                            true
                        } else {
                            false
                        }
                    // must make symbols[1..], terminals[1..]
                    } else {
                        false
                    }
                }
                None => false,
            },
            Symbol::Nonterminal(nonterminal) => {
                println!("Symbol[{}]: {:?}", syms_index, nonterminal,);
                match_symbols(&nonterminal.symbols(), syms_index, terminals, terms_index)
            }
        },
        None => symbols.len() == syms_index && terminals.len() == terms_index,
    }
}
