use crate::commands::CommandRegistry;
use std::io::Write;

pub struct Shell {
    command_registry: CommandRegistry,
}

impl Shell {
    pub fn new() -> Self {
        let command_registry = CommandRegistry::new();

        Self { command_registry }
    }

    pub fn execute_command(&self, input: &str) -> bool {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            return true;
        }

        let cmd_name = parts[0];
        let args = &parts[1..];

        if let Some(cmd) = self.command_registry.get(cmd_name) {
            return cmd.execute(args, &self.command_registry).unwrap_or_else(|err| {
                println!("Error: {}", err);
                true
            });
        }

        println!("{}: command not found", cmd_name);
        true
    }

    pub fn run(&self) {
        loop {
            let mut input = String::new();
            print!("$ ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut input).unwrap();

            if !self.execute_command(&input) {
                break;
            }
        }
    }
}
