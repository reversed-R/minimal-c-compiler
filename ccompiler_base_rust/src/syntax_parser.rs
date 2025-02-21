use std::collections::VecDeque;

use symbol::{Symbol, TerminalTrait};

use crate::tokenizer::token::Token;

pub mod symbol;

pub fn parse() -> () {}

pub fn match_symbols(symbols: VecDeque<Symbol>, terminals: VecDeque<Token>) -> bool {
    match symbols.get(0) {
        Some(symbol) => match symbol {
            Symbol::Terminal(terminal) => match terminals.get(0) {
                Some(token) => {
                    if terminal.contains(&token.kind) {
                        match_symbols(symbols, terminals) // must make symbols[1..], terminals[1..]
                    } else {
                        false
                    }
                }
                None => false,
            },
            Symbol::Nonterminal(nonterminal) => false, //match_symbols(nonterminal.symbols(), terminals),
        },
        None => false,
    }
}
