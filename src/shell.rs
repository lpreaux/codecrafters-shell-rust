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

        // First check built-in commands
        if let Some(cmd) = self.command_registry.get(cmd_name) {
            return cmd
                .execute(args, &self.command_registry)
                .unwrap_or_else(|err| {
                    println!("{}", err);
                    true
                });
        }

        // Then check PATH
        if let Some(_) = crate::utils::path::find_executable_in_path(cmd_name) {
            return match std::process::Command::new(&cmd_name).args(args).status() {
                Ok(_) => true,
                Err(e) => {
                    println!("Error executing {}: {}", cmd_name, e);
                    true
                }
            };
        }

        println!("{}: command not found", cmd_name);
        true
    }

    pub fn run(&self) {
        loop {
            let mut input = String::new();

            // Show current directory in prompt
           /* let current_dir = std::env::current_dir()
            .ok()
            .and_then(|path| path.file_name()?.to_str().map(String::from))
            .unwrap_or_else(|| "shell".to_string());

            print!("{}$ ", current_dir);*/

            print!("$ ");

            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut input).unwrap();

            if !self.execute_command(&input) {
                break;
            }
        }
    }
}
