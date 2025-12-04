use std::io::Write;
use crate::command::CommandHandler;
use crate::commands::CommandRegistry;
use anyhow::Result;
use crate::execution::RedirectionManager;

pub struct HelpHandler;

impl CommandHandler for HelpHandler {
    fn name(&self) -> &'static str {
        "help"
    }

    fn execute(&self,
               args: &[String],
               registry: &CommandRegistry,
               redirections: &mut RedirectionManager,
    ) -> Result<bool> {
        if let Some(cmd_name) = args.get(0) {
            if let Some(handler) = registry.get(cmd_name) {
                writeln!(redirections.stdout(), "{}", handler.help())?;
            } else {
                writeln!(redirections.stderr(), "Unknown command: {}", cmd_name)?;
            }
        } else {
            writeln!(redirections.stdout(), "Available commands:")?;
            for cmd_name in registry.list_commands() {
                if let Some(handler) = registry.get(cmd_name) {
                    writeln!(redirections.stdout(), "  {}", handler.help())?;
                }
            }
        }
        Ok(true)
    }

    fn help(&self) -> &'static str {
        "help [command] - Show help for all commands or a specific command"
    }
}
