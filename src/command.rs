use crate::commands::CommandRegistry;
use anyhow::Result;

pub trait CommandHandler {
    fn name(&self) -> &'static str;
    fn execute(&self, args: &[&str], registry: &CommandRegistry) -> Result<bool>;
    fn help(&self) -> &'static str;
}
