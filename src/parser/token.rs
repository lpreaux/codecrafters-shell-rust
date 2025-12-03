pub enum Token {
    Word(String),
    Whitespace,
    QuotedString(String, char),
    RedirectOperator(char, String),
}
