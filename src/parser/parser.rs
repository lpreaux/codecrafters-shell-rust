use anyhow::{Result, Context, bail};
use crate::parser::lexer::Lexer;
use crate::parser::Token;

pub struct Parser;

#[derive(Debug)]
pub struct ParsedCommand {
    pub name: String,
    pub args: Vec<String>,
    pub redirections: Vec<(String, String)>,
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

        let mut args: Vec<String> = Vec::new();
        let mut current_arg = String::new();
        let mut redirections: Vec<(String, String)> = Vec::new();

        while let Some(token) = iter.peek() {
            match token {
                Token::RedirectOperator('>', ref output_type) => {
                    // Save current argument if any
                    if !current_arg.is_empty() {
                        args.push(current_arg.clone());
                        current_arg.clear();
                    }

                    let output_type = output_type.clone();
                    iter.next(); // consume the operator

                    // Skip whitespace after '>'
                    while matches!(iter.peek(), Some(Token::Whitespace)) {
                        iter.next();
                    }

                    // Get filename
                    match iter.next() {
                        Some(Token::Word(filename)) => {
                            redirections.push((output_type, filename));
                        }
                        Some(Token::QuotedString(filename, _)) => {
                            redirections.push((output_type, filename));
                        }
                        _ => bail!("Expected filename after '>'"),
                    }

                    // Skip whitespace after filename
                    while matches!(iter.peek(), Some(Token::Whitespace)) {
                        iter.next();
                    }
                }
                Token::Whitespace => {
                    if !current_arg.is_empty() {
                        args.push(current_arg.clone());
                        current_arg.clear();
                    }
                    iter.next();

                    while matches!(iter.peek(), Some(Token::Whitespace)) {
                        iter.next();
                    }
                }
                Token::Word(word) => {
                    current_arg.push_str(word);
                    iter.next();
                }
                Token::QuotedString(content, _) => {
                    current_arg.push_str(content);
                    iter.next();
                }
                _ => {
                    iter.next();
                }
            }
        }

        if !current_arg.is_empty() {
            args.push(current_arg);
        }

        Ok(ParsedCommand {
            name,
            args,
            redirections,
        })
    }
}