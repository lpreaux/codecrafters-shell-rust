pub(crate) enum Token {
    Word(String),
    Whitespace,
    QuotedString(String),
}
