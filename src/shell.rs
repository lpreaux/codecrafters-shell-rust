use std::io::{self, Read, Write};
use std::os::unix::io::AsRawFd;
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

use crate::commands::CommandRegistry;
use crate::parser::{FileDescriptor, Parser, RedirectMode, Redirection};
use crate::utils::path::find_executables_with_prefix;
use std::fs::OpenOptions;
use std::process::Command;
use crate::execution::RedirectionManager;

pub struct Shell {
    command_registry: CommandRegistry,
    original_termios: Option<Termios>,
    last_autocomplete_input: Option<String>,
}

impl Shell {
    pub fn new() -> Self {
        let command_registry = CommandRegistry::new();

        Self {
            command_registry,
            original_termios: None,
            last_autocomplete_input: None,
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
                        self.last_autocomplete_input = None;
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
                        self.last_autocomplete_input = None;
                    }
                    c if c >= 32 && c < 127 => {
                        // Caractère imprimable
                        let ch = c as char;
                        input.push(ch);
                        print!("{}", ch);
                        io::stdout().flush().unwrap();
                        self.last_autocomplete_input = None;
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
        let stdout_redirect = command.stdout_redirect();
        let stderr_redirect = command.stderr_redirect();

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
        stdout_redirect: Option<&Redirection>,
        stderr_redirect: Option<&Redirection>,
    ) -> anyhow::Result<bool> {
        let mut redirections = RedirectionManager::with_redirections(
            stdout_redirect,
            stderr_redirect,
        )?;

        cmd.execute(args, &self.command_registry, &mut redirections)
    }

    fn execute_external(
        &self,
        name: &str,
        args: &[String],
        stdout_redirect: Option<&Redirection>,
        stderr_redirect: Option<&Redirection>,
    ) -> anyhow::Result<bool> {
        let mut cmd = Command::new(name);
        cmd.args(args);

        if let Some(redir) = stdout_redirect {
            if let Some(parent) = std::path::Path::new(&redir.target).parent() {
                std::fs::create_dir_all(parent)?;
            }
            cmd.stdout(
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(redir.is_append())
                    .truncate(redir.is_overwrite())
                    .open(&redir.target)?,
            );
        }

        if let Some(redir) = stderr_redirect {
            if let Some(parent) = std::path::Path::new(&redir.target).parent() {
                std::fs::create_dir_all(parent)?;
            }
            cmd.stderr(
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(redir.is_append())
                    .truncate(redir.is_overwrite())
                    .open(&redir.target)?,
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

    fn handle_autocomplete(&mut self, input: &mut String) {
        let parts: Vec<&str> = input.split_whitespace().collect();

        // On complète seulement si c'est le premier mot (la commande)
        if parts.len() > 1 {
            return;
        }

        let prefix = parts.get(0).unwrap_or(&"");

        let mut matches =
            self.command_registry.find_command_starting_with(prefix);
        matches.extend(find_executables_with_prefix(prefix));
        matches.sort();
        matches.dedup();

        match matches.len() {
            0 => {
                // Aucune correspondance
                self.ring_bell();
                self.last_autocomplete_input = None;
            }
            1 => {
                // Une seule correspondance : compléter avec un espace
                self.complete_input(input, &format!("{} ", matches[0]));
                self.last_autocomplete_input = None;
            }
            _ => {
                // Plusieurs correspondances
                self.handle_multiple_matches(input, &matches);
            }
        }
    }

    fn handle_multiple_matches(&mut self, input: &mut String, matches: &[String]) {
        let lcp = Self::find_longest_common_prefix(matches);

        // Si le LCP est plus long que l'input actuel, le compléter
        if lcp.len() > input.len() {
            self.complete_input(input, &lcp);
        }

        // Vérifier si c'est le deuxième TAB consécutif
        if self.last_autocomplete_input.as_ref() == Some(input) {
            // Deuxième TAB : afficher toutes les options
            self.display_matches(matches, input);
            self.last_autocomplete_input = None;
        } else {
            // Premier TAB : sonner la cloche
            self.ring_bell();
            self.last_autocomplete_input = Some(input.clone());
        }
    }

    fn complete_input(&self, input: &mut String, completion: &str) {
        // Effacer l'input actuel visuellement
        for _ in 0..input.len() {
            print!("\x08 \x08");
        }

        // Remplacer par la complétion
        *input = completion.to_string();
        print!("{}", input);
        io::stdout().flush().unwrap();
    }

    fn display_matches(&self, matches: &[String], current_input: &str) {
        println!();
        for cmd in matches {
            print!("{}  ", cmd);
        }
        println!();
        print!("$ {}", current_input);
        io::stdout().flush().unwrap();
    }

    fn ring_bell(&self) {
        print!("\x07");
        io::stdout().flush().unwrap();
    }

    fn find_longest_common_prefix(strings: &[String]) -> String {
        if strings.is_empty() {
            return String::new();
        }

        if strings.len() == 1 {
            return strings[0].clone();
        }

        let first = &strings[0];
        let mut prefix_len = first.len();

        for s in &strings[1..] {
            prefix_len = prefix_len.min(s.len());
            prefix_len = first
                .chars()
                .zip(s.chars())
                .take(prefix_len)
                .take_while(|(a, b)| a == b)
                .count();

            if prefix_len == 0 {
                break;
            }
        }

        first.chars().take(prefix_len).collect()
    }
}

impl Drop for Shell {
    fn drop(&mut self) {
        let _ = self.disable_raw_mode();
    }
}
