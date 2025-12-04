use crate::command::CommandHandler;
use crate::commands::CommandRegistry;
use anyhow::Result;
use crate::execution::RedirectionManager;

pub struct ExitHandler;

impl CommandHandler for ExitHandler {
    fn name(&self) -> &'static str {
        "exit"
    }

    fn execute(&self,
               args: &[String],
               _registry: &CommandRegistry,
               _redirections: &mut RedirectionManager,
    ) -> Result<bool> {
        let code = args.get(0).and_then(|s| s.parse().ok()).unwrap_or(0);
        std::process::exit(code);
    }

    fn help(&self) -> &'static str {
        "exit [code] - Exit the shell with optional exit code"
    }
}
