use crate::parser::Redirection;
use anyhow::Result;
use std::fs::OpenOptions;
use std::io::{self, Write};

/// Structure pour gérer les redirections stdout/stderr
pub struct RedirectionManager {
    stdout: Box<dyn Write>,
    stderr: Box<dyn Write>,
}

impl RedirectionManager {
    /// Crée un nouveau gestionnaire de redirections avec stdout et stderr standard
    pub fn new() -> Self {
        Self {
            stdout: Box::new(io::stdout()),
            stderr: Box::new(io::stderr()),
        }
    }

    /// Configure les redirections en fonction des paramètres fournis
    pub fn with_redirections(
        stdout_redirect: Option<&Redirection>,
        stderr_redirect: Option<&Redirection>,
    ) -> Result<Self> {
        let stdout = Self::open_redirect(stdout_redirect)?
            .unwrap_or_else(|| Box::new(io::stdout()));

        let stderr = Self::open_redirect(stderr_redirect)?
            .unwrap_or_else(|| Box::new(io::stdout()));

        Ok(Self { stdout, stderr })
    }

    /// Ouvre un fichier pour une redirection donnée
    fn open_redirect(redirect: Option<&Redirection>) -> Result<Option<Box<dyn Write>>> {
        match redirect {
            Some(redir) => {
                // Créer les dossiers parents si nécessaire
                if let Some(parent) = std::path::Path::new(&redir.target).parent() {
                    std::fs::create_dir_all(parent)?;
                }

                let file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(redir.is_append())
                    .truncate(redir.is_overwrite())
                    .open(&redir.target)?;

                Ok(Some(Box::new(file)))
            }
            None => Ok(None),
        }
    }

    /// Retourne une référence mutable vers stdout
    pub fn stdout(&mut self) -> &mut dyn Write {
        &mut *self.stdout
    }

    /// Retourne une référence mutable vers stderr
    pub fn stderr(&mut self) -> &mut dyn Write {
        &mut *self.stderr
    }
}
