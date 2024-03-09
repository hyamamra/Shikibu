use super::token::Token;
use crate::interpreter::{error::SyntaxError, tokens::symbol::Symbol};

pub struct Scanner {
    chars: Vec<char>,
    cursor: usize,
}

impl Iterator for Scanner {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(c) = self.peek() else {
            return None;
        };

        Some(match c {
            x if x.is_symbol() => self.drain_symbol().unwrap(),
            '「' => self.drain_string().unwrap(),
            '0'..='9' | '０'..='９' => self.drain_number(),
            ' ' | '　' | '\t' => self.drain_spaces_and_tabs(),
            '\r' | '\n' => self.drain_newline(),
            '#' | '＃' => self.drain_comment(),
            _ => self.drain_keyword_or_identifier().unwrap(),
        })
    }
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            chars: source.chars().collect(),
            cursor: 0,
        }
    }

    fn peek(&self) -> Option<&char> {
        self.chars.get(self.cursor)
    }

    fn next(&mut self) -> Option<char> {
        if self.cursor < self.chars.len() {
            let c = self.chars[self.cursor];
            self.cursor += 1;
            Some(c)
        } else {
            None
        }
    }

    fn drain_symbol(&mut self) -> Result<Token, SyntaxError> {
        let position = self.cursor;
        let c1 = self.next().unwrap();
        let c2 = self.peek().unwrap_or(&' ');

        Ok(Token::symbol(
            match c1 {
                '+' | '＋' => Symbol::Plus,
                '-' | '－' | '―' | 'ー' | '‐' => Symbol::Minus,
                '*' | '＊' | '×' => Symbol::Asterisk,
                '/' | '／' => Symbol::Slash,
                '=' | '＝' => match c2 {
                    '=' | '＝' => {
                        _ = self.next();
                        Symbol::EqualEqual
                    }
                    _ => Symbol::Equal,
                },
                '!' | '！' => match c2 {
                    '=' | '＝' => {
                        _ = self.next();
                        Symbol::BangEqual
                    }
                    _ => Symbol::Bang,
                },
                '<' | '＜' => match c2 {
                    '=' | '＝' => {
                        _ = self.next();
                        Symbol::LessEqual
                    }
                    _ => Symbol::Less,
                },
                '>' | '＞' => match c2 {
                    '=' | '＝' => {
                        _ = self.next();
                        Symbol::GreaterEqual
                    }
                    _ => Symbol::Greater,
                },
                '(' | '（' => Symbol::OpenParen,
                ')' | '）' => Symbol::CloseParen,
                ',' | '，' | '、' => Symbol::Comma,
                '~' | '～' => Symbol::Tilde,
                _ => return Err(SyntaxError::invalid_char(c1, position)),
            },
            position,
        ))
    }

    fn drain_string(&mut self) -> Result<Token, SyntaxError> {
        let mut string = String::new();
        _ = self.next();
        let position = self.cursor;

        while let Some(c) = self.peek() {
            if c == &'\\' || c == &'￥' {
                _ = self.next();
                if let Some(_) = self.peek() {
                    string.push(self.next().unwrap());
                }
                continue;
            }
            if c == &'」' {
                _ = self.next();
                break;
            }
            string.push(self.next().unwrap());
        }
        Ok(Token::string(string, position))
    }

    fn drain_digits_as_string(&mut self) -> String {
        let mut digits = String::new();
        while let Some(c) = self.peek() {
            match c {
                '0'..='9' | '０'..='９' => digits.push(self.next().unwrap()),
                _ => break,
            }
        }
        digits
    }

    fn drain_number(&mut self) -> Token {
        let position = self.cursor;
        let mut number = String::from(self.drain_digits_as_string().as_str());

        let Some(c) = self.peek() else {
            return Token::number(number, position);
        };
        if c == &'.' || c == &'．' {
            number.push(self.next().unwrap());
            number.push_str(self.drain_digits_as_string().as_str());
        }
        Token::number(number, position)
    }

    fn drain_spaces_and_tabs(&mut self) -> Token {
        let position = self.cursor;
        let mut spaces = String::new();

        while let Some(c) = self.peek() {
            match c {
                ' ' | '　' | '\t' => {
                    spaces.push(self.next().unwrap());
                }
                _ => break,
            }
        }
        Token::spaces(spaces.len(), position)
    }

    fn drain_newline(&mut self) -> Token {
        self.cursor += 1;
        Token::newline()
    }

    fn drain_comment(&mut self) -> Token {
        let position = self.cursor;

        while let Some(c) = self.peek() {
            match c {
                '\r' | '\n' => break,
                _ => _ = self.next().unwrap(),
            }
        }
        Token::comment(position)
    }

    fn drain_keyword_or_identifier(&mut self) -> Result<Token, SyntaxError> {
        if let Some(c) = self.peek() {
            if c.is_special() {
                return Err(SyntaxError::invalid_char(*c, self.cursor));
            }
        }

        let position = self.cursor;
        let mut identifier = String::new();

        while let Some(c) = self.peek() {
            match c {
                x if x.is_symbol() | x.is_special() => break,
                _ => identifier.push(self.next().unwrap()),
            }
        }
        Ok(if let Ok(keyword) = identifier.parse() {
            Token::keyword(keyword, position)
        } else {
            Token::identifier(identifier, position)
        })
    }
}

trait IsSymbol {
    fn is_symbol(&self) -> bool;
}

impl IsSymbol for char {
    fn is_symbol(&self) -> bool {
        match self {
            '+' | '＋' | '-' | '－' | '―' | 'ー' | '‐' | '*' | '＊' | '×' | '/' | '／' | '='
            | '＝' | '!' | '！' | '<' | '＜' | '>' | '＞' | '&' | '＆' | '|' | '｜' | '('
            | '（' | ')' | '）' | '・' | '\\' | '￥' | '~' | '～' | ':' | '：' | ';' | '；'
            | ',' | '，' | '、' | '@' | '＠' | '$' | '＄' | '%' | '％' | '^' | '＾' | '.'
            | '。' | '\'' | '’' | '"' | '”' | '?' | '？' | '[' | ']' | '{' | '｛' | '}' | '｝'
            | '`' | '‘' | '♯' => true,
            _ => false,
        }
    }
}

trait IsSpecial {
    fn is_special(&self) -> bool;
}

impl IsSpecial for char {
    fn is_special(&self) -> bool {
        match self {
            ' ' | '　' | '\t' | '\r' | '\n' | '「' | '」' | '#' | '＃' => true,
            _ => false,
        }
    }
}
