use std::collections::HashMap;

enum JsonValue {
    JsonInt(i64),
    JsonFloat(f64),
    JsonString(String),
    JsonList(Vec<JsonValue>),
    JsonObject(HashMap<String, JsonValue>),
}

enum Token {
    LiteralInt(i64),
    LiteralFloat(f64),
    LiteralString(String),
    Ident(String),
    Comma,
    Colon,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Eof,
}

fn tokenize(content: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = content.chars();

    loop {
        match chars.next() {
            None => {
                tokens.push(Token::Eof);
                return tokens;
            }
            Some(char) => match char {
                _ => tokens.push(Token::Colon),
            },
        }
    }
}

fn main() {}
