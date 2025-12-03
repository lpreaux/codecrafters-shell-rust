use crate::commands::CommandRegistry;
use crate::parser::{Parser, RedirectMode};
use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;
use std::process::{Command, Stdio};

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
            .find(|(fd, _, _)| fd == "stdout");

        let stderr_redirect = command
            .redirections
            .iter()
            .find(|(fd, _, _)| fd == "stderr");

        // Commandes internes
        if let Some(cmd) = self.command_registry.get(&command.name) {
            return self
                .execute_builtin(cmd, &command.args, stdout_redirect, stderr_redirect)
                .unwrap_or_else(|err| {
                    println!("{}", err);
                    true
                });
        }

        // Commandes externes
        if crate::utils::path::find_executable_in_path(&command.name).is_some() {
            return self
                .execute_external(
                    &command.name,
                    &command.args,
                    stdout_redirect,
                    stderr_redirect,
                )
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
        stdout_redirect: Option<&(String, String, RedirectMode)>,
        stderr_redirect: Option<&(String, String, RedirectMode)>,
    ) -> anyhow::Result<bool> {
        let mut stdout: Box<dyn Write> = match stdout_redirect {
            Some((_, filename, mode)) => {
                if let Some(parent) = std::path::Path::new(filename).parent() {
                    std::fs::create_dir_all(parent)?;
                }
                Box::new(
                    OpenOptions::new()
                        .write(true)
                        .create(true)
                        .append(matches!(mode, RedirectMode::Append))
                        .truncate(matches!(mode, RedirectMode::Overwrite))
                        .open(filename)?
                )
            }
            None => Box::new(io::stdout()),
        };

        let mut stderr: Box<dyn Write> = match stderr_redirect {
            Some((_, filename, mode)) => {
                if let Some(parent) = std::path::Path::new(filename).parent() {
                    std::fs::create_dir_all(parent)?;
                }
                Box::new(
                    OpenOptions::new()
                        .write(true)
                        .create(true)
                        .append(matches!(mode, RedirectMode::Append))
                        .truncate(matches!(mode, RedirectMode::Overwrite))
                        .open(filename)?
                )
            }
            None => Box::new(io::stderr()),
        };

        cmd.execute(args, &self.command_registry, &mut *stdout, &mut *stderr)
    }

    fn execute_external(
        &self,
        name: &str,
        args: &[String],
        stdout_redirect: Option<&(String, String, RedirectMode)>,
        stderr_redirect: Option<&(String, String, RedirectMode)>,
    ) -> anyhow::Result<bool> {
        let mut cmd = Command::new(name);
        cmd.args(args);

        if let Some((_, filename, mode)) = stdout_redirect {
            if let Some(parent) = std::path::Path::new(filename).parent() {
                std::fs::create_dir_all(parent)?;
            }
            cmd.stdout(
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(matches!(mode, RedirectMode::Append))
                    .truncate(matches!(mode, RedirectMode::Overwrite))
                    .open(filename)?
            );
        }

        if let Some((_, filename, mode)) = stderr_redirect {
            if let Some(parent) = std::path::Path::new(filename).parent() {
                std::fs::create_dir_all(parent)?;
            }
            cmd.stderr(
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(matches!(mode, RedirectMode::Append))
                    .truncate(matches!(mode, RedirectMode::Overwrite))
                    .open(filename)?
            );
        }

        cmd.status()?;
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
