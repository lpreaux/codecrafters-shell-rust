use std::io::Write;
use crate::command::CommandHandler;
use crate::commands::CommandRegistry;
use anyhow::Result;

pub struct HelpHandler;

impl CommandHandler for HelpHandler {
    fn name(&self) -> &'static str {
        "help"
    }

    fn execute(&self,
               args: &[String],
               registry: &CommandRegistry,
               stdout: &mut dyn Write,
               stderr: &mut dyn Write,
    ) -> Result<bool> {
        if let Some(cmd_name) = args.get(0) {
            if let Some(handler) = registry.get(cmd_name) {
                writeln!(stdout, "{}", handler.help())?;
            } else {
                writeln!(stderr, "Unknown command: {}", cmd_name)?;
            }
        } else {
            writeln!(stdout, "Available commands:")?;
            for cmd_name in registry.list_commands() {
                if let Some(handler) = registry.get(cmd_name) {
                    writeln!(stdout, "  {}", handler.help())?;
                }
            }
        }
        Ok(true)
    }

    fn help(&self) -> &'static str {
        "help [command] - Show help for all commands or a specific command"
    }
}
