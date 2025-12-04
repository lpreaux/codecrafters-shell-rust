use crate::command::CommandHandler;
use crate::commands::CommandRegistry;
use anyhow::Result;
use crate::execution::RedirectionManager;

pub struct EchoHandler;

impl CommandHandler for EchoHandler {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn execute(&self,
               args: &[String],
               _registry: &CommandRegistry,
               redirections: &mut RedirectionManager,
    ) -> Result<bool> {
        writeln!(redirections.stdout(), "{}", args.join(" "))?;
        Ok(true)
    }

    fn help(&self) -> &'static str {
        "echo [text] - Print text to stdout"
    }
}
