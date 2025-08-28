use std::cmp::PartialEq;
use crate::parser::token;
use crate::parser::token::Token;

pub struct Lexer;

#[derive(PartialEq)]
enum LexerState {
    Default,
    QuotedString,
}


impl Lexer {
    pub fn lex(input: &str) {
        let mut tokens: Vec<token::Token> = Vec::new();
        let mut state: LexerState = LexerState::Default;
        let mut curr: &str = "";
        for char in input.split("") {
            match char {
                " " => {
                    if !curr.is_empty() && LexerState::Default.eq(&state) {
                        tokens.push(Token::Word(curr.to_string()))
                        curr = "";
                    }
                    tokens.push(Token::Whitespace)
                },
                "\"" | "\'" => {
                    if curr.is_empty() {
                        curr += char.to_string().as_str();
                    } else {
                        tokens.push(Token::QuotedString(curr.to_string()));
                        curr = "";
                    }
                },
                _ => curr += char.to_string().as_str();
            }
        }
    }
}