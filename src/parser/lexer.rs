use anyhow::{Result, bail};
use crate::parser::token::Token;

pub struct Lexer;

#[derive(PartialEq, Debug)]
enum LexerState {
    Default,
    Escaped,
    SingleQuoted,
    DoubleQuoted,
    EscapedInDoubleQuote,
}

impl Lexer {
    pub fn lex(input: &str) -> Result<Vec<Token>> {
        // Estimation : en moyenne ~25% des caractères deviennent des tokens
        let mut tokens = Vec::with_capacity(input.len() / 4);
        let mut state = LexerState::Default;
        // Capacité initiale raisonnable pour un mot/argument moyen
        let mut curr = String::with_capacity(32);

        for ch in input.chars() {
            state = match (state, ch) {
                // État Escaped (hors quotes)
                (LexerState::Escaped, ch) => {
                    curr.push(ch);
                    LexerState::Default
                }

                // État EscapedInDoubleQuote
                (LexerState::EscapedInDoubleQuote, ch) => {
                    match ch {
                        '"' | '\\' => {
                            // Caractères spéciaux : échappés
                            curr.push(ch);
                        }
                        _ => {
                            // Autres caractères : \ reste littéral
                            curr.push('\\');
                            curr.push(ch);
                        }
                    }
                    LexerState::DoubleQuoted
                }

                // Default
                (LexerState::Default, ' ') => {
                    Self::push_word_if_not_empty(&mut tokens, &mut curr);
                    tokens.push(Token::Whitespace);
                    LexerState::Default
                }
                (LexerState::Default, '\\') => LexerState::Escaped,
                (LexerState::Default, '\'') => LexerState::SingleQuoted,
                (LexerState::Default, '"') => LexerState::DoubleQuoted,
                (LexerState::Default, '\n') => LexerState::Default,
                (LexerState::Default, '>') => {
                    tokens.push(Token::Operator('>'));
                    LexerState::Default
                }
                (LexerState::Default, ch) => {
                    curr.push(ch);
                    LexerState::Default
                }

                // Single Quoted
                (LexerState::SingleQuoted, '\'') => {
                    tokens.push(Token::QuotedString(curr.clone(), '\''));
                    curr.clear();
                    LexerState::Default
                }
                (LexerState::SingleQuoted, ch) => {
                    curr.push(ch);
                    LexerState::SingleQuoted
                }

                // Double Quoted
                (LexerState::DoubleQuoted, '"') => {
                    tokens.push(Token::QuotedString(curr.clone(), '"'));
                    curr.clear();
                    LexerState::Default
                }
                (LexerState::DoubleQuoted, '\\') => {
                    LexerState::EscapedInDoubleQuote
                }
                (LexerState::DoubleQuoted, ch) => {
                    curr.push(ch);
                    LexerState::DoubleQuoted
                }
            };
        }

        // Vérifications finales
        match state {
            LexerState::SingleQuoted => bail!("Unclosed single quote"),
            LexerState::DoubleQuoted => bail!("Unclosed double quote"),
            LexerState::EscapedInDoubleQuote => {
                bail!("Trailing backslash in double quote")
            }
            LexerState::Escaped => bail!("Trailing backslash"),
            LexerState::Default => {
                Self::push_word_if_not_empty(&mut tokens, &mut curr);
            }
        }

        Ok(tokens)
    }

    #[inline]
    fn push_word_if_not_empty(tokens: &mut Vec<Token>, curr: &mut String) {
        if !curr.is_empty() {
            tokens.push(Token::Word(curr.clone()));
            curr.clear();
        }
    }
}