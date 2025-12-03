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
               output: &mut dyn Write,
    ) -> Result<bool> {
        if let Some(cmd_name) = args.get(0) {
            if let Some(handler) = registry.get(cmd_name) {
                writeln!(output, "{}", handler.help())?;
            } else {
                writeln!(output, "Unknown command: {}", cmd_name)?;
            }
        } else {
            writeln!(output, "Available commands:")?;
            for cmd_name in registry.list_commands() {
                if let Some(handler) = registry.get(cmd_name) {
                    writeln!(output, "  {}", handler.help())?;
                }
            }
        }
        Ok(true)
    }

    fn help(&self) -> &'static str {
        "help [command] - Show help for all commands or a specific command"
    }
}
