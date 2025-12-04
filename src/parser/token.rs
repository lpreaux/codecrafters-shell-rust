pub(crate) use crate::parser::redirection::{FileDescriptor, RedirectMode};

pub enum Token {
    Litteral(String),
    Whitespace,
    QuotedString(String, char),
    Redirect { mode: RedirectMode, fd: FileDescriptor },
}
