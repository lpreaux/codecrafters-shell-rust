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
               stdout: &mut dyn Write,
               stderr: &mut dyn Write,
    ) -> Result<bool> {
        if args.len() != 1 {
            writeln!(stderr, "type takes exactly one argument")?;
            return Ok(true);
        }

        let cmd_name = &args[0];
        let builtin_commands: Vec<&str> = registry.list_commands();

        if builtin_commands.contains(&cmd_name.as_str()) {
            writeln!(stdout, "{} is a shell builtin", cmd_name)?;
            return Ok(true);
        }

        if let Some(path) = crate::utils::path::find_executable_in_path(cmd_name) {
            writeln!(stdout, "{} is {}", cmd_name, path.to_str().unwrap())?;
        } else {
            writeln!(stderr, "{}: not found", cmd_name)?;
        }

        Ok(true)
    }

    fn help(&self) -> &'static str {
        "type [command] - Show the type of a command"
    }
}
