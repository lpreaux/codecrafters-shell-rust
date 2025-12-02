use anyhow::{Result, bail };
use std::cmp::PartialEq;
use crate::parser::token::Token;

pub struct Lexer;

#[derive(PartialEq)]
enum LexerState {
    Default,
    QuotedString(char),
}


impl Lexer {
    pub fn lex(input: &str) -> Result<Vec<Token>> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut state: LexerState = LexerState::Default;
        let mut curr = String::new();

        for ch in input.chars() {
            match ch {
                ' ' => {
                    match state {
                        LexerState::QuotedString(_) => curr.push(ch),
                        LexerState::Default => {
                            if !curr.is_empty() && LexerState::Default.eq(&state) {
                                tokens.push(Token::Word(curr.clone()));
                                curr.clear();
                            }
                            tokens.push(Token::Whitespace)
                        }
                    }
                },
                '"' | '\'' => {
                    match state {
                        LexerState::Default => state = LexerState::QuotedString(ch),
                        LexerState::QuotedString(quote_char) => {
                            if quote_char == ch { // Vérifie que c'est le même type de quote
                                tokens.push(Token::QuotedString(
                                    curr.clone(),
                                    quote_char
                                ));
                                curr.clear();
                                state = LexerState::Default;
                            } else {
                                curr.push(ch); // Quote différent à l'intérieur
                            }
                        }
                    }
                },
                '\n' => continue,
                _ => curr.push(ch),
            }
        }

        // Vérifier qu'il n'y a pas de quote non fermée
        match state {
            LexerState::QuotedString(quote_char) => {
                bail!("Unclosed quote: {}", quote_char);
            },
            LexerState::Default => {
                if !curr.is_empty() {
                    tokens.push(Token::Word(curr));
                }
            }
        }

        Ok(tokens)
    }
}