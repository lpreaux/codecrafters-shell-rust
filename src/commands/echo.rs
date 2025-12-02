use crate::command::CommandHandler;
use crate::commands::CommandRegistry;
use anyhow::Result;

pub struct EchoHandler;

impl CommandHandler for EchoHandler {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn execute(&self, args: &[String], _registry: &CommandRegistry) -> Result<bool> {
        println!("{}", args.join(" "));
        Ok(true)
    }

    fn help(&self) -> &'static str {
        "echo [text] - Print text to stdout"
    }
}
