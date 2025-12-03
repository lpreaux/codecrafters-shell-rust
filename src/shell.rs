use std::fs::File;
use std::io;
use crate::commands::CommandRegistry;
use std::io::Write;
use std::process::{Command, Stdio};
use crate::parser::Parser;

pub struct Shell {
    command_registry: CommandRegistry,
}

impl Shell {
    pub fn new() -> Self {
        let command_registry = CommandRegistry::new();

        Self { command_registry }
    }

    pub fn execute_command(&self, input: &str) -> bool {
        let command = match Parser::parse(input) {
            Ok(command) => command,
            Err(err) => {
                println!("Error parsing command: {}", err);
                return true;
            }
        };

        // Déterminer la destination de sortie
        let stdout_redirect = command
            .redirections
            .iter()
            .find(|(fd, _)| fd == "stdout"); // Ton parser utilise "stdin" pour stdout

        // Commandes internes
        if let Some(cmd) = self.command_registry.get(&command.name) {
            return self
                .execute_builtin(cmd, &command.args, stdout_redirect)
                .unwrap_or_else(|err| {
                    println!("{}", err);
                    true
                });
        }

        // Commandes externes
        if crate::utils::path::find_executable_in_path(&command.name).is_some() {
            return self
                .execute_external(&command.name, &command.args, stdout_redirect)
                .unwrap_or_else(|err| {
                    println!("Error executing {}: {}", command.name, err);
                    true
                });
        }

        println!("{}: command not found", command.name);
        true
    }

    fn execute_builtin(
        &self,
        cmd: &Box<dyn crate::command::CommandHandler>,
        args: &[String],
        redirect: Option<&(String, String)>,
    ) -> anyhow::Result<bool> {
        match redirect {
            Some((_, filename)) => {
                let mut file = File::create(filename)?;
                cmd.execute(args, &self.command_registry, &mut file)
            }
            None => {
                let mut stdout = io::stdout();
                cmd.execute(args, &self.command_registry, &mut stdout)
            }
        }
    }

    fn execute_external(
        &self,
        name: &str,
        args: &[String],
        redirect: Option<&(String, String)>,
    ) -> anyhow::Result<bool> {
        let mut cmd = Command::new(name);
        cmd.args(args);

        if let Some((_, filename)) = redirect {
            let file = File::create(filename)?;
            cmd.stdout(Stdio::from(file));
        }

        // Important: stderr reste sur le terminal pour les erreurs
        let status = cmd.status()?;

        // Toujours retourner true pour continuer le shell
        // même si la commande échoue
        Ok(true)
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


