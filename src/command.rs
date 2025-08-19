use anyhow::Result;

pub trait CommandHandler {
    fn name(&self) -> &'static str;
    fn execute(&self, args: &[&str]) -> Result<bool>;
    fn help(&self) -> &'static str;
}
