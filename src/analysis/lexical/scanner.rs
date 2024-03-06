use super::token::Token;
use crate::analysis::lexical::symbol::Symbol;

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
            '+' | '＋' | '-' | '－' | '*' | '＊' | '/' | '／' | '=' | '＝' | '!' | '！' | '<'
            | '＜' | '>' | '＞' | '&' | '＆' | '|' | '｜' | '(' | '（' | ')' | '）' | '・'
            | '\\' | '￥' => self.drain_symbol(),
            '"' | '”' => self.drain_string(),
            '1'..='9' | '１'..='９' => self.drain_number(),
            ' ' | '　' | '\t' => self.drain_spaces_and_tabs(),
            '\r' | '\n' => self.drain_newline(),
            '#' | '＃' => self.drain_comment(),
            _ => self.drain_keyword_or_identifier(),
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

    fn drain_symbol(&mut self) -> Token {
        let position = self.cursor;
        let c1 = self.next().unwrap();
        let c2 = self.peek().unwrap_or(&' ');

        Token::symbol(
            match c1 {
                '+' | '＋' => Symbol::Plus,
                '-' | '－' => Symbol::Minus,
                '*' | '＊' => Symbol::Star,
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
                '(' | '（' => Symbol::LeftParen,
                ')' | '）' => Symbol::RightParen,
                '&' | '＆' => Symbol::Ampersand,
                '|' | '｜' => Symbol::Pipe,
                ',' | '，' | '、' => Symbol::Comma,
                '\\' | '￥' => Symbol::Backslash,
                '・' => Symbol::Bullet,
                _ => panic!(),
            },
            position,
        )
    }

    fn drain_string(&mut self) -> Token {
        let position = self.cursor;
        let mut string = String::new();
        _ = self.next();

        while let Some(c) = self.peek() {
            if c == &'\\' || c == &'￥' {
                _ = self.next();
                if let Some(_) = self.peek() {
                    string.push(self.next().unwrap());
                }
                continue;
            }
            if c == &'"' || c == &'”' {
                _ = self.next();
                break;
            }
            string.push(self.next().unwrap());
        }
        Token::string(string, position)
    }

    fn drain_digits_as_string(&mut self) -> String {
        let mut digits = String::new();

        while let Some(c) = self.peek() {
            if c.is_single_or_multi_byte_digit() {
                digits.push(self.next().unwrap());
            } else {
                break;
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
            number.push_str(self.drain_digits_as_string().as_str());
            _ = self.next();
        }
        Token::number(number, position)
    }

    fn drain_spaces_and_tabs(&mut self) -> Token {
        let position = self.cursor;
        let mut spaces = String::new();

        while let Some(c) = self.peek() {
            if c.is_single_or_multi_byte_space_or_tab() {
                spaces.push(self.next().unwrap());
            } else {
                break;
            }
        }
        Token::spaces(spaces, position)
    }

    fn drain_newline(&mut self) -> Token {
        let position = self.cursor;
        let newline = self.next().unwrap().to_string();
        Token::newline(newline, position)
    }

    fn drain_comment(&mut self) -> Token {
        let position = self.cursor;
        let mut comment = String::new();

        while let Some(c) = self.peek() {
            if c.is_newline() {
                break;
            }
            comment.push(self.next().unwrap());
        }
        Token::comment(comment, position)
    }

    fn drain_keyword_or_identifier(&mut self) -> Token {
        let position = self.cursor;
        let mut identifier = String::new();

        while let Some(c) = self.peek() {
            if c.is_special_symbol() || c.is_single_or_multi_byte_space_or_tab() || c.is_newline() {
                break;
            }
            identifier.push(self.next().unwrap());
        }
        if let Ok(keyword) = identifier.parse() {
            Token::keyword(keyword, position)
        } else {
            Token::identifier(identifier, position)
        }
    }
}

trait IsSpecialSymbol {
    fn is_special_symbol(&self) -> bool;
}

impl IsSpecialSymbol for char {
    fn is_special_symbol(&self) -> bool {
        match self {
            '=' | '＝' | '+' | '＋' | '-' | '－' | '*' | '＊' | '/' | '／' | '!' | '！' | '<'
            | '＜' | '>' | '＞' | '&' | '＆' | '|' | '｜' | '(' | '（' | ')' | '）' | '"' | '”'
            | ' ' | '　' | '\t' | '\r' | '\n' | '#' | '＃' | ',' | '，' | '、' | '・' | '\\'
            | '￥' => true,
            _ => false,
        }
    }
}

trait IsSingleOrMultiByteDigit {
    fn is_single_or_multi_byte_digit(&self) -> bool;
}

impl IsSingleOrMultiByteDigit for char {
    fn is_single_or_multi_byte_digit(&self) -> bool {
        match self {
            '1'..='9' | '１'..='９' => true,
            _ => false,
        }
    }
}

trait IsSingleOrMultiBytespaceOrTab {
    fn is_single_or_multi_byte_space_or_tab(&self) -> bool;
}

impl IsSingleOrMultiBytespaceOrTab for char {
    fn is_single_or_multi_byte_space_or_tab(&self) -> bool {
        match self {
            ' ' | '　' | '\t' => true,
            _ => false,
        }
    }
}

trait IsNewline {
    fn is_newline(&self) -> bool;
}

impl IsNewline for char {
    fn is_newline(&self) -> bool {
        match self {
            '\r' | '\n' => true,
            _ => false,
        }
    }
}
