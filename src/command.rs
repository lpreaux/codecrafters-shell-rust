use std::io::Write;
use crate::commands::CommandRegistry;
use anyhow::Result;

pub trait CommandHandler {
    fn name(&self) -> &'static str;
    fn execute(
        &self,
        args: &[String],
        registry: &CommandRegistry,
        output: &mut dyn Write,
    ) -> Result<bool>;
    fn help(&self) -> &'static str;
}
