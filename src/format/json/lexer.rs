use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(String),
    String(String),
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Comma,
    True,
    False,
    Null,
    Invalid(char),
    Eof,
}

fn collect_number(first: char, iter: &mut Peekable<Chars<'_>>) -> Token {
    let mut res = String::new();
    res.push(first);

    if first == '-' {
        let Some(&d) = iter.peek() else {
            return Token::Invalid('-');
        };
        if !d.is_ascii_digit() {
            return Token::Invalid('-');
        }
        res.push(d);
        iter.next();
    }

    loop {
        match iter.peek() {
            Some(&d) if d.is_ascii_digit() || d == '.' || d == 'e' || d == 'E' => {
                res.push(d);
                iter.next();
            }
            Some(&('+' | '-')) if res.ends_with('e') || res.ends_with('E') => {
                res.push(iter.next().unwrap());
            }
            _ => break,
        }
    }

    Token::Number(res)
}

fn collect_string(iter: &mut Peekable<Chars<'_>>) -> Token {
    let mut res = String::new();

    loop {
        let Some(c) = iter.next() else {
            return Token::String(res);
        };

        match c {
            '"' => return Token::String(res),
            '\\' => {
                let Some(escaped) = iter.next() else {
                    return Token::String(res);
                };
                match escaped {
                    '"' => res.push('"'),
                    '\\' => res.push('\\'),
                    '/' => res.push('/'),
                    'n' => res.push('\n'),
                    'r' => res.push('\r'),
                    't' => res.push('\t'),
                    other => {
                        res.push('\\');
                        res.push(other);
                    }
                }
            }
            c => res.push(c),
        }
    }
}

fn collect_ident(first: char, iter: &mut Peekable<Chars<'_>>) -> Token {
    let mut res = String::new();
    res.push(first);

    loop {
        match iter.peek() {
            Some(&c) if c.is_alphabetic() => {
                res.push(c);
                iter.next();
            }
            _ => break,
        }
    }

    match res.as_str() {
        "true" => Token::True,
        "false" => Token::False,
        "null" => Token::Null,
        _ => Token::Invalid(first),
    }
}

pub fn tokenize(content: &str) -> Vec<Token> {
    let mut iter = content.chars().peekable();
    let mut tokens = Vec::new();

    loop {
        let Some(c) = iter.next() else {
            tokens.push(Token::Eof);
            return tokens;
        };

        tokens.push(match c {
            c if c.is_whitespace() => continue,
            '"' => collect_string(&mut iter),
            '-' | '0'..='9' => collect_number(c, &mut iter),
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            ':' => Token::Colon,
            ',' => Token::Comma,
            c if c.is_alphabetic() => collect_ident(c, &mut iter),
            c => Token::Invalid(c),
        })
    }
}

#[test]
fn test_collect_number() {
    let mut content = "234.1234".chars().peekable();
    assert_eq!(
        collect_number('1', &mut content),
        Token::Number("1234.1234".into())
    );

    let mut content = "1234.1234".chars().peekable();
    assert_eq!(
        collect_number('-', &mut content),
        Token::Number("-1234.1234".into())
    );
}

#[test]
fn test_collect_string() {
    let content = "this is a string\"";
    assert_eq!(
        collect_string(&mut content.chars().peekable()),
        Token::String("this is a string".into())
    );
}

#[test]
fn test_tokenize() {
    let content = r#"{"name": "John", "age": 30}"#;

    let tokens = tokenize(content);

    let expected = vec![
        Token::LBrace,
        Token::String("name".into()),
        Token::Colon,
        Token::String("John".into()),
        Token::Comma,
        Token::String("age".into()),
        Token::Colon,
        Token::Number("30".into()),
        Token::RBrace,
        Token::Eof,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_tokenize_keywords() {
    let tokens = tokenize("true false null");
    assert_eq!(
        tokens,
        vec![Token::True, Token::False, Token::Null, Token::Eof]
    );
}
