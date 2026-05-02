use std::iter::Peekable;
use std::slice::Iter;

use crate::JsonValue;
use crate::json::lexer::{Token, tokenize};

type Tokens<'a> = Peekable<Iter<'a, Token>>;

pub fn parse(content: &str) -> Result<JsonValue, String> {
    let tokens = tokenize(content);
    let mut iter = tokens.iter().peekable();
    parse_value(&mut iter)
}

fn parse_value(tokens: &mut Tokens<'_>) -> Result<JsonValue, String> {
    let Some(token) = tokens.next() else {
        return Err("unexpected end of input".into());
    };

    match token {
        Token::Null => Ok(JsonValue::JsonNull),
        Token::True => Ok(JsonValue::JsonBool(true)),
        Token::False => Ok(JsonValue::JsonBool(false)),
        Token::Number(s) => parse_number(s),
        Token::String(s) => Ok(JsonValue::JsonString(s.clone())),
        Token::LBrace => parse_object(tokens),
        Token::LBracket => parse_array(tokens),
        Token::Eof => Err("unexpected end of input".into()),
        _ => Err("unexpected token".into()),
    }
}

fn parse_number(s: &str) -> Result<JsonValue, String> {
    if s.contains('.') || s.contains('e') || s.contains('E') {
        let Ok(f) = s.parse::<f64>() else {
            return Err(format!("invalid float: {s}"));
        };
        Ok(JsonValue::JsonFloat(f))
    } else if s.starts_with('-') {
        let Ok(i) = s.parse::<i64>() else {
            return Err(format!("invalid integer: {s}"));
        };
        Ok(JsonValue::JsonInt(i))
    } else {
        let Ok(u) = s.parse::<u64>() else {
            return Err(format!("invalid integer: {s}"));
        };
        Ok(JsonValue::JsonUint(u))
    }
}

fn parse_object(tokens: &mut Tokens<'_>) -> Result<JsonValue, String> {
    let mut fields: Vec<(String, JsonValue)> = Vec::new();

    loop {
        match tokens.peek() {
            Some(Token::RBrace) => {
                tokens.next();
                break;
            }
            Some(Token::Eof) | None => return Err("unterminated object".into()),
            _ => {}
        }

        let Some(Token::String(key)) = tokens.next() else {
            return Err("expected string key".into());
        };
        let key = key.clone();

        let Some(Token::Colon) = tokens.next() else {
            return Err("expected ':' after object key".into());
        };

        let value = parse_value(tokens)?;
        fields.push((key, value));

        match tokens.peek() {
            Some(Token::Comma) => {
                tokens.next();
            }
            Some(Token::RBrace) => {}
            _ => return Err("expected ',' or '}' in object".into()),
        }
    }

    Ok(JsonValue::JsonObject(fields))
}

fn parse_array(tokens: &mut Tokens<'_>) -> Result<JsonValue, String> {
    let mut items = Vec::new();

    loop {
        match tokens.peek() {
            Some(Token::RBracket) => {
                tokens.next();
                break;
            }
            Some(Token::Eof) | None => return Err("unterminated array".into()),
            _ => {}
        }

        let value = parse_value(tokens)?;
        items.push(value);

        match tokens.peek() {
            Some(Token::Comma) => {
                tokens.next();
            }
            Some(Token::RBracket) => {}
            _ => return Err("expected ',' or ']' in array".into()),
        }
    }

    Ok(JsonValue::JsonList(items))
}

#[test]
fn test_parse_primitives() {
    assert!(matches!(parse("null"), Ok(JsonValue::JsonNull)));
    assert!(matches!(parse("true"), Ok(JsonValue::JsonBool(true))));
    assert!(matches!(parse("false"), Ok(JsonValue::JsonBool(false))));
    assert!(matches!(parse("42"), Ok(JsonValue::JsonUint(42))));
    assert!(matches!(parse("-7"), Ok(JsonValue::JsonInt(-7))));
    assert!(matches!(parse("3.14"), Ok(JsonValue::JsonFloat(_))));
    assert!(matches!(parse(r#""hello""#), Ok(JsonValue::JsonString(_))));
}

#[test]
fn test_parse_object() {
    let Ok(JsonValue::JsonObject(fields)) = parse(r#"{"name": "Alice", "age": 30}"#) else {
        panic!("expected object");
    };
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].0, "name");
    assert!(matches!(&fields[0].1, JsonValue::JsonString(s) if s == "Alice"));
    assert_eq!(fields[1].0, "age");
    assert!(matches!(fields[1].1, JsonValue::JsonUint(30)));
}

#[test]
fn test_parse_array() {
    let Ok(JsonValue::JsonList(items)) = parse("[1, 2, 3]") else {
        panic!("expected array");
    };
    assert_eq!(items.len(), 3);
}

#[test]
fn test_parse_nested() {
    let json = r#"{"users": [{"id": 1, "active": true}, {"id": 2, "active": false}]}"#;
    let Ok(JsonValue::JsonObject(fields)) = parse(json) else {
        panic!("expected object");
    };
    assert_eq!(fields[0].0, "users");
    let JsonValue::JsonList(users) = &fields[0].1 else {
        panic!("expected array");
    };
    assert_eq!(users.len(), 2);
}

#[test]
fn test_parse_empty_structures() {
    assert!(matches!(parse("{}"), Ok(JsonValue::JsonObject(v)) if v.is_empty()));
    assert!(matches!(parse("[]"), Ok(JsonValue::JsonList(v)) if v.is_empty()));
}
