use crate::command::CommandHandler;
use crate::commands::CommandRegistry;
use crate::execution::RedirectionManager;
use anyhow::Result;

pub struct TypeHandler;

impl CommandHandler for TypeHandler {
    fn name(&self) -> &'static str {
        "type"
    }

    fn execute(&self,
               args: &[String],
               registry: &CommandRegistry,
               redirections: &mut RedirectionManager,
    ) -> Result<bool> {
        if args.len() != 1 {
            writeln!(redirections.stderr(), "type takes exactly one argument")?;
            return Ok(true);
        }

        let cmd_name = &args[0];
        let builtin_commands: Vec<&str> = registry.list_commands();

        if builtin_commands.contains(&cmd_name.as_str()) {
            writeln!(redirections.stdout(), "{} is a shell builtin", cmd_name)?;
            return Ok(true);
        }

        if let Some(path) = crate::utils::path::find_executable_in_path(cmd_name) {
            writeln!(redirections.stdout(), "{} is {}", cmd_name, path.to_str().unwrap())?;
        } else {
            writeln!(redirections.stderr(), "{}: not found", cmd_name)?;
        }

        Ok(true)
    }

    fn help(&self) -> &'static str {
        "type [command] - Show the type of a command"
    }
}
