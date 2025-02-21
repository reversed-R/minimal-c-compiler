use symbol::{Expr, NonterminalTrait, Stat, Symbol, TerminalTrait};

use crate::tokenizer::token::Token;

pub mod symbol;

pub fn parse(tokens: Vec<Token>) -> bool {
    let mut result = false;
    if match_symbols(&Expr::Calc.symbols(), 0, &tokens, 0) {
        println!("Expr::Calc");
        result = true;
    }
    if match_symbols(&Stat::ExprStat.symbols(), 0, &tokens, 0) {
        println!("Stat::ExprStat");
        result = true;
    }

    result
}

pub fn match_symbols(
    symbols: &Vec<Symbol>,
    syms_index: usize,
    terminals: &Vec<Token>,
    terms_index: usize,
) -> bool {
    match symbols.get(syms_index) {
        Some(symbol) => match symbol {
            Symbol::Terminal(terminal) => match terminals.get(0) {
                Some(token) => {
                    if terminal.contains(&token.kind) {
                        match_symbols(symbols, syms_index + 1, terminals, terms_index + 1)
                    // must make symbols[1..], terminals[1..]
                    } else {
                        false
                    }
                }
                None => false,
            },
            Symbol::Nonterminal(nonterminal) => {
                match_symbols(&nonterminal.symbols(), syms_index, terminals, terms_index)
            }
        },
        None => false,
    }
}
