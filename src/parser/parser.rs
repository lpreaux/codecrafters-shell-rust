use anyhow::{Result, Context, bail};
use crate::parser::lexer::Lexer;
use crate::parser::Token;

pub struct Parser;

#[derive(Debug)]
pub struct ParsedCommand {
    pub name: String,
    pub args: Vec<String>,
}

impl Parser {
    pub fn parse(input: &str) -> Result<ParsedCommand> {
        let tokens = Lexer::lex(input)
            .context("Failed to tokenize input")?;

        Self::parse_tokens(tokens)
    }

    fn parse_tokens(tokens: Vec<Token>) -> Result<ParsedCommand> {
        let mut iter = tokens.into_iter().peekable();

        // Skip leading whitespace
        while matches!(iter.peek(), Some(Token::Whitespace)) {
            iter.next();
        }

        // Parse command name
        let name = match iter.next() {
            Some(Token::Word(name)) => name,
            Some(Token::QuotedString(name, _)) => name,
            Some(_) => bail!("First token must be a word or quoted string"),
            None => bail!("No command provided"),
        };

        // Skip whitespace after command name
        while matches!(iter.peek(), Some(Token::Whitespace)) {
            iter.next();
        }

        // Parse arguments (grouping consecutive non-whitespace tokens)
        let mut args: Vec<String> = Vec::new();
        let mut current_arg = String::new();

        while let Some(token) = iter.next() {
            match token {
                Token::Whitespace => {
                    // Whitespace separates arguments
                    if !current_arg.is_empty() {
                        args.push(current_arg.clone());
                        current_arg.clear();
                    }
                    // Skip consecutive whitespaces
                    while matches!(iter.peek(), Some(Token::Whitespace)) {
                        iter.next();
                    }
                }
                Token::Word(word) => {
                    current_arg.push_str(&word)
                }
                Token::QuotedString(content, '\'') => {
                    current_arg.push_str(&content);
                }
                Token::QuotedString(content, '"') => {
                    current_arg.push_str(&content);
                }
                _ => bail!("Unexpected token"),
            }

        }

        // Don't forget the last argument if any
        if !current_arg.is_empty() {
            args.push(current_arg);
        }

        Ok(ParsedCommand {
            name,
            args,
        })
    }
}