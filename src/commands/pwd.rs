use std::io::Write;
use crate::command::CommandHandler;
use crate::commands::CommandRegistry;
use anyhow::Result;
use crate::execution::RedirectionManager;

pub struct PwdHandler;

impl CommandHandler for PwdHandler {
    fn name(&self) -> &'static str {
        "pwd"
    }

    fn execute(&self,
               _args: &[String],
               _registry: &CommandRegistry,
               redirections: &mut RedirectionManager,
    ) -> Result<bool> {
        writeln!(redirections.stdout(), "{}", std::env::current_dir()?.display())?;
        Ok(true)
    }

    fn help(&self) -> &'static str {
        "pwd - Print the current working directory"
    }
}
