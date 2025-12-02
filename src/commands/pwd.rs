use crate::command::CommandHandler;
use crate::commands::CommandRegistry;
use anyhow::Result;

pub struct PwdHandler;

impl CommandHandler for PwdHandler {
    fn name(&self) -> &'static str {
        "pwd"
    }

    fn execute(&self, _args: &[String], _registry: &CommandRegistry) -> Result<bool> {
        println!("{}", std::env::current_dir()?.display());
        Ok(true)
    }

    fn help(&self) -> &'static str {
        "pwd - Print the current working directory"
    }
}
