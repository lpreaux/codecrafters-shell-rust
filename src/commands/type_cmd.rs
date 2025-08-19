use crate::command::CommandHandler;
use crate::commands::CommandRegistry;
use anyhow::{anyhow, Result};

pub struct TypeHandler;

impl CommandHandler for TypeHandler {
    fn name(&self) -> &'static str {
        "type"
    }

    fn execute(&self, args: &[&str], registry: &CommandRegistry) -> Result<bool> {
        if args.len() != 1 {
            return Err(anyhow!("type takes exactly one argument"));
        }

        let builtin_commands: Vec<&str> = registry.list_commands();
        if builtin_commands.contains(&args[0]) {
            println!("{} is a shell builtin", args[0]);
        } else {
            println!("{}: not found", args[0]);
        }
        Ok(true)
    }

    fn help(&self) -> &'static str {
        "type [command] - Show the type of a command"
    }
}
