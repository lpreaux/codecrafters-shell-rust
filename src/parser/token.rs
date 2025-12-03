pub enum Token {
    Word(String),
    Whitespace,
    QuotedString(String, char),
    Operator(char),
}
