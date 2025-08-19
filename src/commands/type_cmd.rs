use crate::command::CommandHandler;
use crate::commands::CommandRegistry;
use anyhow::{anyhow, Result};
use std::fs::File;
use std::os::unix::fs::PermissionsExt;

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
            return Ok(true);
        }

        for path in std::env::split_paths(&std::env::var("PATH")?) {
            let path = path.join(args[0]);
            if path.exists() {
                if path.is_file() {
                    let permissions = File::open(&path)?.metadata()?.permissions();
                    if permissions.mode() & 0o111 != 0 {
                        println!("{} is {}", args[0], path.to_str().unwrap());
                        return Ok(true);
                    }
                }
            }
        }

        println!("{}: not found", args[0]);
        Ok(true)
    }

    fn help(&self) -> &'static str {
        "type [command] - Show the type of a command"
    }
}
