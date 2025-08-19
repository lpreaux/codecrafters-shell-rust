use crate::command::CommandHandler;
use anyhow::Result;

pub struct EchoHandler;

impl CommandHandler for EchoHandler {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn execute(&self, args: &[&str]) -> Result<bool> {
        println!("{}", args.join(" "));
        Ok(true)
    }

    fn help(&self) -> &'static str {
        "echo [text] - Print text to stdout"
    }
}
