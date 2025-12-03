pub enum Token {
    Word(String),
    Whitespace,
    QuotedString(String, char),
    Redirect { mode: RedirectMode, fd: String },
}

#[derive(Debug, Clone)]
pub enum RedirectMode {
    Overwrite,  // >
    Append,     // >>
}