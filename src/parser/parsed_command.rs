use crate::parser::redirection::Redirection;

#[derive(Debug)]
pub struct ParsedCommand {
    pub name: String,
    pub args: Vec<String>,
    pub redirections: Vec<Redirection>,
}

impl ParsedCommand {
    pub fn stdout_redirect(&self) -> Option<&Redirection> {
        self.redirections
            .iter()
            .find(|r| r.is_stdout())
    }

    pub fn stderr_redirect(&self) -> Option<&Redirection> {
        self.redirections
            .iter()
            .find(|r| r.is_stderr())
    }
}