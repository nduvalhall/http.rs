use std::iter::Peekable;
use std::slice::Iter;

use crate::format::json::Json;
use crate::format::json::lexer::{Token, tokenize};

type Tokens<'a> = Peekable<Iter<'a, Token>>;

pub fn parse(content: &str) -> Result<Json, String> {
    let tokens = tokenize(content);
    let mut iter = tokens.iter().peekable();
    parse_value(&mut iter)
}

fn parse_value(tokens: &mut Tokens<'_>) -> Result<Json, String> {
    let Some(token) = tokens.next() else {
        return Err("unexpected end of input".into());
    };

    match token {
        Token::Null => Ok(Json::Null),
        Token::True => Ok(Json::Bool(true)),
        Token::False => Ok(Json::Bool(false)),
        Token::Number(s) => parse_number(s),
        Token::String(s) => Ok(Json::String(s.clone())),
        Token::LBrace => parse_object(tokens),
        Token::LBracket => parse_array(tokens),
        Token::Eof => Err("unexpected end of input".into()),
        _ => Err("unexpected token".into()),
    }
}

fn parse_number(s: &str) -> Result<Json, String> {
    let Ok(f) = s.parse::<f64>() else {
        return Err(format!("invalid number: {s}"));
    };
    Ok(Json::Number(f))
}

fn parse_object(tokens: &mut Tokens<'_>) -> Result<Json, String> {
    let mut fields: Vec<(String, Json)> = Vec::new();

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

    Ok(Json::object(fields))
}

fn parse_array(tokens: &mut Tokens<'_>) -> Result<Json, String> {
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

    Ok(Json::Array(items))
}

#[test]
fn test_parse_primitives() {
    assert!(matches!(parse("null"), Ok(Json::Null)));
    assert!(matches!(parse("true"), Ok(Json::Bool(true))));
    assert!(matches!(parse("false"), Ok(Json::Bool(false))));
    assert!(matches!(parse("42"), Ok(Json::Number(n)) if n == 42.0));
    assert!(matches!(parse("-7"), Ok(Json::Number(n)) if n == -7.0));
    assert!(matches!(parse("3.14"), Ok(Json::Number(_))));
    assert!(matches!(parse(r#""hello""#), Ok(Json::String(_))));
}

#[test]
fn test_parse_object() {
    let Ok(Json::Object(obj)) = parse(r#"{"name": "Alice", "age": 30}"#) else {
        panic!("expected object");
    };
    assert_eq!(obj.0.len(), 2);
    assert!(matches!(obj.field("name"), Some(Json::String(s)) if s == "Alice"));
    assert!(matches!(obj.field("age"), Some(Json::Number(n)) if *n == 30.0));
}

#[test]
fn test_parse_array() {
    let Ok(Json::Array(items)) = parse("[1, 2, 3]") else {
        panic!("expected array");
    };
    assert_eq!(items.len(), 3);
}

#[test]
fn test_parse_nested() {
    let json = r#"{"users": [{"id": 1, "active": true}, {"id": 2, "active": false}]}"#;
    let Ok(Json::Object(obj)) = parse(json) else {
        panic!("expected object");
    };
    let Some(Json::Array(users)) = obj.field("users") else {
        panic!("expected array");
    };
    assert_eq!(users.len(), 2);
}

#[test]
fn test_parse_empty_structures() {
    assert!(matches!(parse("{}"), Ok(Json::Object(o)) if o.0.is_empty()));
    assert!(matches!(parse("[]"), Ok(Json::Array(v)) if v.is_empty()));
}
