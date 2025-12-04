pub(crate) use crate::parser::redirection::{FileDescriptor, RedirectMode};

pub enum Token {
    Word(String),
    Whitespace,
    QuotedString(String, char),
    Redirect { mode: RedirectMode, fd: FileDescriptor },
}
