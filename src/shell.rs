use std::io::{self, Read, Write};
use std::os::unix::io::AsRawFd;
use termios::{tcgetattr, tcsetattr, Termios, ECHO, ICANON, TCSANOW};

use crate::commands::CommandRegistry;
use crate::parser::{Parser, RedirectMode};
use std::fs::OpenOptions;
use std::process::Command;

pub struct Shell {
    command_registry: CommandRegistry,
    original_termios: Option<Termios>,
}

impl Shell {
    pub fn new() -> Self {
        let command_registry = CommandRegistry::new();

        Self {
            command_registry,
            original_termios: None,
        }
    }

    pub fn run(&mut self) {
        self.enable_raw_mode().unwrap();

        loop {
            let mut input = String::new();

            print!("$ ");
            io::stdout().flush().unwrap();

            // Boucle de lecture caractère par caractère
            loop {
                let mut buffer = [0u8; 1];
                io::stdin().read_exact(&mut buffer).unwrap();

                match buffer[0] {
                    b'\n' | b'\r' => {
                        // Enter : fin de saisie
                        println!();
                        break;
                    }
                    b'\t' => {
                        // Tab : autocompletion
                        self.handle_autocomplete(&mut input);
                    }
                    127 | 8 => {
                        // Backspace (127 sur Linux, 8 sur certains systèmes)
                        if !input.is_empty() {
                            input.pop();
                            print!("\x08 \x08"); // Efface visuellement
                            io::stdout().flush().unwrap();
                        }
                    }
                    c if c >= 32 && c < 127 => {
                        // Caractère imprimable
                        let ch = c as char;
                        input.push(ch);
                        print!("{}", ch);
                        io::stdout().flush().unwrap();
                    }
                    _ => {
                        // Ignorer les autres caractères (séquences escape, etc.)
                    }
                }
            }

            if !input.trim().is_empty() {
                if !self.execute_command(&input) {
                    break;
                }
            }
        }

        self.disable_raw_mode().unwrap();
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
                        .open(filename)?,
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
                        .open(filename)?,
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
                    .open(filename)?,
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
                    .open(filename)?,
            );
        }

        cmd.status()?;
        Ok(true)
    }

    fn enable_raw_mode(&mut self) -> io::Result<()> {
        let fd = io::stdin().as_raw_fd();

        // Récupérer et sauvegarder les paramètres originaux
        let original = Termios::from_fd(fd)?;
        self.original_termios = Some(original);

        // Créer une copie modifiée pour le mode raw
        let mut raw = original;
        raw.c_lflag &= !(ICANON | ECHO);

        // Appliquer les nouveaux paramètres
        tcsetattr(fd, TCSANOW, &raw)?;
        Ok(())
    }

    fn disable_raw_mode(&self) -> io::Result<()> {
        if let Some(original) = &self.original_termios {
            let fd = io::stdin().as_raw_fd();
            tcsetattr(fd, TCSANOW, original)?;
        }
        Ok(())
    }

    fn handle_autocomplete(&self, input: &mut String) {
        let parts: Vec<&str> = input.split_whitespace().collect();

        // On complete seulement si c'est le premier mot (la commande)
        if parts.len() <= 1 {
            let prefix = parts.get(0).unwrap_or(&"");
            let matches = self.command_registry.find_command_starting_with(prefix);

            match matches.len() {
                0 => {
                    // Aucune correspondance
                }
                1 => {
                    // Une seule correspondance : compléter
                    let completion = matches[0];

                    // Effacer l'input actuel visuellement
                    for _ in 0..input.len() {
                        print!("\x08 \x08");
                    }

                    // Remplacer par la complétion
                    *input = completion.to_string();
                    print!("{} ", input);
                    io::stdout().flush().unwrap();
                }
                _ => {
                    // Plusieurs correspondances : afficher les options
                    println!();
                    for cmd in &matches {
                        println!("{}", cmd);
                    }
                    print!("$ {}", input);
                    io::stdout().flush().unwrap();
                }
            }
        }
    }
}

impl Drop for Shell {
    fn drop(&mut self) {
        let _ = self.disable_raw_mode();
    }
}
