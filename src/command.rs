use crate::commands::CommandRegistry;
use anyhow::Result;
use crate::execution::RedirectionManager;

pub trait CommandHandler {
    fn name(&self) -> &'static str;
    fn execute(
        &self,
        args: &[String],
        registry: &CommandRegistry,
        redirections: &mut RedirectionManager,
    ) -> Result<bool>;
    fn help(&self) -> &'static str;
}
