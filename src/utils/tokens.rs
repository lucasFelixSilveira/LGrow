use regex::Regex;
use crate::language::analizers::lexical::Token;

#[derive(Debug)]
pub enum TokenType {
    Number,
    Operator,
    Identifier,
    String,
    // Adicione outros tipos de tokens, se necessário
}

pub fn determine_token_type(token: &Token) -> TokenType {
    let content = &token.content;

    let number_regex = Regex::new(r"^\d+$").unwrap();
    let identifier_regex = Regex::new(r"^[a-zA-Z_]\w*$").unwrap();
    let string_regex = Regex::new(r#"^".*"$"#).unwrap();

    if number_regex.is_match(&content) {
        TokenType::Number
    } else if identifier_regex.is_match(&content) {
        TokenType::Identifier
    } else if string_regex.is_match(&content) {
        TokenType::String
    } else {
        TokenType::Operator
    }
}