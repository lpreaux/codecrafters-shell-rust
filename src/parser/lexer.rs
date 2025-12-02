use anyhow::{Result, bail};
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
        let mut escape_next = false;
        let mut curr = String::new();

        for ch in input.chars() {
            // Gestion de l'échappement SEULEMENT hors quotes
            if escape_next && state == LexerState::Default {
                curr.push(ch);
                escape_next = false;
                continue;
            }

            match ch {
                ' ' => {
                    match state {
                        LexerState::QuotedString(_) => curr.push(ch),
                        LexerState::Default => {
                            if !curr.is_empty() {
                                tokens.push(Token::Word(curr.clone()));
                                curr.clear();
                            }
                            tokens.push(Token::Whitespace)
                        }
                    }
                },
                '"' | '\'' => {
                    match state {
                        LexerState::Default => {
                            state = LexerState::QuotedString(ch);
                        },
                        LexerState::QuotedString(quote_char) => {
                            if quote_char == ch {
                                tokens.push(Token::QuotedString(
                                    curr.clone(),
                                    quote_char
                                ));
                                curr.clear();
                                state = LexerState::Default;
                            } else {
                                curr.push(ch);
                            }
                        }
                    }
                },
                '\\' => {
                    match state {
                        LexerState::Default => {
                            escape_next = true;
                        },
                        LexerState::QuotedString(_) => {
                            // Dans les quotes, \ est littéral pour l'instant
                            curr.push(ch);
                        }
                    }
                },
                '\n' => {
                    if state != LexerState::Default {
                        curr.push(ch);
                    }
                    // Hors quotes, on ignore les newlines
                },
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