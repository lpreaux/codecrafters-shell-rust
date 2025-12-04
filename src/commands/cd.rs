use std::path::PathBuf;
use crate::command::CommandHandler;
use crate::commands::CommandRegistry;
use anyhow::Result;
use crate::execution::RedirectionManager;

pub struct CdHandler;

impl CommandHandler for CdHandler {
    fn name(&self) -> &'static str {
        "cd"
    }

    fn execute(&self,
               args: &[String],
               _registry: &CommandRegistry,
               redirections: &mut RedirectionManager,
    ) -> Result<bool> {
        let target_dir = if args.is_empty() {
            std::env::var("HOME").unwrap_or_else(|_| "/".to_string())
        } else if args.len() == 1 {
            args[0].to_string()
        } else {
            writeln!(redirections.stderr(), "cd: too many arguments")?;
            return Ok(true);
        };

        let new_path = if target_dir.starts_with('/') || target_dir.starts_with('~') {
            if target_dir.starts_with('~') {
                let home = std::env::var("HOME")
                    .unwrap_or_else(|_| "/".to_string());
                PathBuf::from(target_dir.replacen('~', &home, 1))
            } else {
                PathBuf::from(&target_dir)
            }
        } else {
            std::env::current_dir()?.join(&target_dir)
        };

        if !new_path.exists() {
            writeln!(redirections.stderr(), "cd: {}: No such file or directory", target_dir)?;
            return Ok(true);
        }

        if !new_path.is_dir() {
            writeln!(redirections.stderr(), "cd: {}: Not a directory", target_dir)?;
            return Ok(true);
        }

        std::env::set_current_dir(new_path)?;
        Ok(true)
    }

    fn help(&self) -> &'static str {
        "cd [directory] - Change the current directory"
    }
}
