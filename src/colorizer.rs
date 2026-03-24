use crate::tokens::{Token, TokenType};

const KEYWORD: &str = "blue; font-weight: bold";
const IDENTIFIER: &str = "black";
const NUMBER: &str = "purple";
const STRING: &str = "orange";
const COMMENT: &str = "grey";
const OPERATOR: &str = "black";

fn token_color(token: &Token) -> &'static str {
    match token.token_type {
        TokenType::EOF => "",
        TokenType::Identifier(_) => IDENTIFIER,
        TokenType::NumberLiteral(_) => NUMBER,
        TokenType::StringLiteral(_) => STRING,
        TokenType::Comment(_) => COMMENT,
        TokenType::Slash
        | TokenType::Star
        | TokenType::Plus
        | TokenType::Minus
        | TokenType::Semicolon
        | TokenType::Comma
        | TokenType::Equal
        | TokenType::OpenParentheses
        | TokenType::CloseParentheses
        | TokenType::OpenBrace
        | TokenType::CloseBrace
        | TokenType::DoubleEqual
        | TokenType::Bang
        | TokenType::BangEqual
        | TokenType::OpenAngleBracket
        | TokenType::CloseAngleBracket
        | TokenType::LessEqual
        | TokenType::GreaterEqual => OPERATOR,
        TokenType::Dot
        | TokenType::And
        | TokenType::Else
        | TokenType::False
        | TokenType::For
        | TokenType::Fun
        | TokenType::If
        | TokenType::Nil
        | TokenType::Or
        | TokenType::Print
        | TokenType::Return
        | TokenType::Class
        | TokenType::Super
        | TokenType::This
        | TokenType::True
        | TokenType::Var
        | TokenType::While => KEYWORD,
        TokenType::NewLine => "",
        TokenType::Space => "",
    }
}

fn token_content(token: &Token) -> &str {
    match &token.token_type {
        TokenType::NewLine => "<br />",
        TokenType::Space => "&nbsp;",
        TokenType::Identifier(s) => s.as_str(),
        TokenType::StringLiteral(s) => unreachable!(),
        TokenType::Comment(s) => unreachable!(),
        TokenType::Slash => "/",
        TokenType::Star => "*",
        TokenType::Plus => "+",
        TokenType::Minus => "-",
        TokenType::Semicolon => ";",
        TokenType::Comma => ",",
        TokenType::Equal => "=",
        TokenType::OpenParentheses => "(",
        TokenType::CloseParentheses => ")",
        TokenType::OpenBrace => "{",
        TokenType::CloseBrace => "}",
        TokenType::DoubleEqual => "==",
        TokenType::Bang => "!",
        TokenType::BangEqual => "!=",
        TokenType::OpenAngleBracket => "<",
        TokenType::CloseAngleBracket => ">",
        TokenType::LessEqual => "<=",
        TokenType::GreaterEqual => ">=",
        TokenType::Dot => ".",
        TokenType::And => "and",
        TokenType::Class => "class",
        TokenType::Else => "else",
        TokenType::False => "false",
        TokenType::For => "for",
        TokenType::Fun => "fun",
        TokenType::If => "if",
        TokenType::Nil => "nil",
        TokenType::Or => "or",
        TokenType::Print => "print",
        TokenType::Return => "return",
        TokenType::Super => "super",
        TokenType::This => "this",
        TokenType::True => "true",
        TokenType::Var => "var",
        TokenType::While => "while",
        TokenType::EOF => "",
        TokenType::NumberLiteral(_) => unreachable!(),
    }
}
use std::fmt::Write;
pub fn generate_html(tokens: Vec<Token>) -> String {
    let mut result = String::new();
    let mut number_buffer = String::new();
    for t in tokens {
        let color = token_color(&t);
        let content = match t.token_type {
            TokenType::NumberLiteral(n) => {
                number_buffer.clear();
                write!(&mut number_buffer, "{}", n).unwrap();
                number_buffer.as_str()
            }
            TokenType::StringLiteral(s) => {
                number_buffer.clear();
                number_buffer.push('"');
                number_buffer.push_str(&s);
                number_buffer.push('"');
                number_buffer.as_str()
            }
            TokenType::Comment(s) => {

                number_buffer.clear();
                number_buffer.push_str("//");
                number_buffer.push_str(&s);
                number_buffer.as_str()
            }
            _ => token_content(&t),
        };

        let span = format!("<span style=\"color:{}\">{}</span>", color, content);
        result.push_str(span.as_str());
    }
    result
}

pub fn colorize_to_file(tokens: Vec<Token>, filename: &str) -> std::io::Result<()> {
    std::fs::write(filename, generate_html(tokens))
}
