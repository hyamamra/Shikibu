use super::{lexeme::Lexeme, token::Token};
use crate::analysis::error::SyntaxError;
use std::collections::VecDeque;

pub fn sanitize(queue: &mut VecDeque<Token>) -> Result<VecDeque<Token>, ()> {
    let mut sanitized = VecDeque::new();
    let mut indents = Indents::from(0);
    cosume_empty_lines(queue).ok();

    // Add a newline token to the end of the queue to ensure that the last line is processed.
    if let Some(_) = queue.back() {
        queue.push_back(Token::newline());
    } else {
        return Ok(sanitized);
    }

    while let Some(token) = queue.front() {
        match token.lexeme {
            Lexeme::Newline => {
                if cosume_empty_lines(queue).is_ok() {
                    sanitized.push_back(Token::newline());
                }
                let Some(front) = queue.front() else {
                    break;
                };
                let mut offside_tokens = generate_offside_tokens(front, &mut indents).unwrap();

                while !offside_tokens.is_empty() {
                    sanitized.push_back(offside_tokens.pop().unwrap());
                }
            }
            Lexeme::Comment => _ = queue.pop_front(),
            Lexeme::Spaces(_) => _ = queue.pop_front(),
            _ => sanitized.push_back(queue.pop_front().unwrap()),
        }
    }
    while !indents.is_top_level() {
        sanitized.push_back(Token::dedent());
        _ = indents.pop();
    }
    Ok(sanitized)
}

fn cosume_empty_lines(queue: &mut VecDeque<Token>) -> Result<(), ()> {
    let mut deletable: Vec<&Token> = Vec::new();

    for index in 0..queue.len() {
        let token = queue.get(index).unwrap();

        match token.lexeme {
            Lexeme::Spaces(_) => deletable.push(token),
            Lexeme::Newline => deletable.push(token),
            Lexeme::Comment => deletable.push(token),
            _ => break,
        }
    }
    deletable.reverse();

    for index in 0..deletable.len() {
        if let Lexeme::Newline = deletable.get(index).unwrap().lexeme {
            queue.drain(..deletable.len() - index);
            return Ok(());
        }
    }
    Err(())
}

fn generate_offside_tokens(
    front: &Token,
    indents: &mut Indents,
) -> Result<Vec<Token>, SyntaxError> {
    let mut offside_tokens: Vec<Token> = Vec::new();

    // If the next token is not a space, it is assumed that there is no indent,
    // and all indentations are removed.
    let Lexeme::Spaces(spaces) = front.lexeme else {
        while indents.len() != 1 {
            _ = indents.pop();
            offside_tokens.push(Token::dedent());
        }
        return Ok(offside_tokens);
    };

    if indents.last().unwrap() < &spaces {
        // Adds an indent token if the indent width is larger than the current width.
        indents.push(&spaces);
        offside_tokens.push(Token::indent());
    } else if &spaces < indents.last().unwrap() {
        // If the indent width is less than the current one,
        // pop from the top of the indent stack until the same width is found,
        // then add as many dedentation tokens as you pop.
        loop {
            // Raise an error if an indent of a width that does not exist on the indent stack is detected.
            let Some(indent_length) = indents.last() else {
                return Err(SyntaxError::unexpected_token(front.clone()));
            };
            if &spaces == indent_length {
                break;
            };
            _ = indents.pop();
            offside_tokens.push(Token::dedent());
        }
    }
    Ok(offside_tokens)
}

struct Indents(Vec<usize>);

impl From<usize> for Indents {
    fn from(len: usize) -> Self {
        let mut indents = Vec::new();
        indents.push(len);
        Self(indents)
    }
}

impl Indents {
    fn last(&self) -> Option<&usize> {
        self.0.last()
    }

    fn push(&mut self, len: &usize) {
        self.0.push(*len)
    }

    fn pop(&mut self) -> Option<usize> {
        self.0.pop()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_top_level(&self) -> bool {
        self.0.last().unwrap() == &0
    }
}
