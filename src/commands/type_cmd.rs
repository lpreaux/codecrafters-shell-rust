use std::io::Write;
use crate::command::CommandHandler;
use crate::commands::CommandRegistry;
use anyhow::{anyhow, Result};

pub struct TypeHandler;

impl CommandHandler for TypeHandler {
    fn name(&self) -> &'static str {
        "type"
    }

    fn execute(&self,
               args: &[String],
               registry: &CommandRegistry,
               output: &mut dyn Write,
    ) -> Result<bool> {
        if args.len() != 1 {
            return Err(anyhow!("type takes exactly one argument"));
        }

        let cmd_name = &args[0];
        let builtin_commands: Vec<&str> = registry.list_commands();

        if builtin_commands.contains(&cmd_name.as_str()) {
            writeln!(output, "{} is a shell builtin", cmd_name)?;
            return Ok(true);
        }

        if let Some(path) = crate::utils::path::find_executable_in_path(cmd_name) {
            writeln!(output, "{} is {}", cmd_name, path.to_str().unwrap())?;
        } else {
            writeln!(output, "{}: not found", cmd_name)?;
        }

        Ok(true)
    }

    fn help(&self) -> &'static str {
        "type [command] - Show the type of a command"
    }
}
