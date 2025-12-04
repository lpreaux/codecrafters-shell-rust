use anyhow::{Result, bail};
use crate::parser::lexer::Lexer;
use crate::parser::parsed_command::ParsedCommand;
use crate::parser::redirection::Redirection;
use crate::parser::Token;

pub struct Parser;

impl Parser {
    pub fn parse(input: &str) -> Result<ParsedCommand> {
        let tokens = Lexer::lex(input)?;

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
        let mut redirections: Vec<Redirection> = Vec::new();

        while let Some(token) = iter.peek() {
            match token {
                Token::Redirect { mode, fd } => {
                    if !current_arg.is_empty() {
                        args.push(current_arg.clone());
                        current_arg.clear();
                    }

                    let redirect_mode = mode.clone();
                    let fd_type = *fd;
                    iter.next();

                    while matches!(iter.peek(), Some(Token::Whitespace)) {
                        iter.next();
                    }

                    match iter.next() {
                        Some(Token::Word(filename)) => {
                            redirections.push(Redirection::new(fd_type, filename, redirect_mode));
                        }
                        Some(Token::QuotedString(filename, _)) => {
                            redirections.push(Redirection::new(fd_type, filename, redirect_mode));
                        }
                        _ => bail!("Expected filename after redirect operator"),
                    }

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