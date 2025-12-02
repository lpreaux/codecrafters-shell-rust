use crate::commands::CommandRegistry;
use anyhow::Result;

pub trait CommandHandler {
    fn name(&self) -> &'static str;
    fn execute(&self, args: &[String], registry: &CommandRegistry) -> Result<bool>;
    fn help(&self) -> &'static str;
}
