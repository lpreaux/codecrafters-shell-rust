#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileDescriptor {
    Stdout,
    Stderr,
}

impl FileDescriptor {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "" | "1" | "stdout" => Ok(Self::Stdout),
            "2" | "stderr" => Ok(Self::Stderr),
            _ => Err(s.to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedirectMode {
    Overwrite,  // >
    Append,     // >>
}

#[derive(Debug, Clone)]
pub struct Redirection {
    pub fd: FileDescriptor,
    pub target: String,
    pub mode: RedirectMode,
}

impl Redirection {
    pub fn new(fd: FileDescriptor, target: String, mode: RedirectMode) -> Self {
        Self { fd, target, mode }
    }

    pub fn is_stdout(&self) -> bool {
        self.fd == FileDescriptor::Stdout
    }

    pub fn is_stderr(&self) -> bool {
        self.fd == FileDescriptor::Stderr
    }

    pub fn is_append(&self) -> bool {
        matches!(self.mode, RedirectMode::Append)
    }

    pub fn is_overwrite(&self) -> bool {
        matches!(self.mode, RedirectMode::Overwrite)
    }
}