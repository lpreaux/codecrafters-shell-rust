use anyhow::Result;
use crate::command::CommandHandler;

pub struct HelpHandler {
    available_commands: Vec<(String, String)>, // (name, help_text)
}

impl HelpHandler {
    pub fn new(registry: &super::CommandRegistry) -> Self {
        let available_commands = registry
            .handlers
            .iter()
            .filter(|(name, _)| name.as_str() != "help") // Don't include help in help
            .map(|(name, handler)| (name.clone(), handler.help().to_string()))
            .collect();

        Self { available_commands }
    }
}

impl CommandHandler for HelpHandler {
    fn name(&self) -> &'static str {
        "help"
    }

    fn execute(&self, args: &[&str]) -> Result<bool> {
        if let Some(cmd_name) = args.get(0) {
            // Show help for specific command
            for (name, help_text) in &self.available_commands {
                if name == cmd_name {
                    println!("{}", help_text);
                    return Ok(true);
                }
            }
            println!("Unknown command: {}", cmd_name);
        } else {
            // Show all commands
            println!("Available commands:");
            for (_, help_text) in &self.available_commands {
                println!("  {}", help_text);
            }
        }
        Ok(true)
    }

    fn help(&self) -> &'static str {
        "help [command] - Show help for all commands or a specific command"
    }
}