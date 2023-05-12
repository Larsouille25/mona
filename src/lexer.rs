use std::error::Error;
use std::iter::Enumerate;
use std::str::Chars;

use crate::error::IllegalCharError;
use crate::token::Token;
use crate::Position;

#[derive(Debug)]
pub struct Lexer<'a> {
    text: String,
    pos: usize, // position in the text
    current_char: Option<char>,
    iter: Option<Enumerate<Chars<'a>>>,
    line: u32,
    filename: String,
    filetext: String,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &String, filename: String) -> Lexer {
        Lexer {
            text: text.to_string(),
            pos: 0,
            current_char: None,
            iter: None,
            line: 1,
            filename,
            filetext: text.to_string(),
        }
    }

    pub fn get_current_char(&self) -> char {
        if self.pos >= self.text.len() {
            return '\0';
        }
        self.current_char.unwrap()
    }

    pub fn make_tokens(&'a mut self) -> Result<Vec<Token>, Box<dyn Error>> {
        let mut tokens = Vec::new();

        self.iter = Some(self.text.chars().enumerate());

        while let Some((mut _idx, mut _ch)) = self.iter.as_mut().unwrap().next() {
            self.pos = _idx;
            match _ch {
                // TODO: include all other whitespaces
                ' ' => {}
                '0'..='9' | '.' => {
                    let num = Self::make_number(&self.text, self.pos);

                    if let Err(err) = num {
                        return Err(err);
                    }

                    let (tok, new_pos) = num.unwrap();

                    for _ in 0..(new_pos.0 - 1) {
                        (_idx, _ch) = self
                            .iter
                            .as_mut()
                            .unwrap()
                            .next()
                            .expect("ERR: running out of bounds")
                    }
                    tokens.push(tok);
                }
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '*' => tokens.push(Token::Mul),
                '/' => tokens.push(Token::Div),
                '(' => tokens.push(Token::LParen),
                ')' => tokens.push(Token::RParen),
                _ => {
                    return Err(Box::new(IllegalCharError::new(
                        format!("`{_ch}`"),
                        Position::new(
                            _idx as u32,
                            self.line,
                            _idx as u32,
                            self.filename.clone(), //TODO: Try to remove .clone()
                            self.filetext.clone(),
                        ),
                    )));
                }
            }
        }

        Ok(tokens)
    }

    fn set_current_char(text: &str, pos: usize) -> Option<char> {
        // TODO: rewrite this function I think it's not very efficient ...
        for (i, c) in text.chars().enumerate() {
            if i == pos {
                return Some(c);
            }
        }
        None
    }

    /// This return a tuple (Token, usize) where Token is either
    /// Token::Int(x) with x as an i32 or
    /// Token::Float(x) with x as an f32
    /// and usize is the lenght of the number
    pub fn make_number(text: &str, pos: usize) -> Result<(Token, (usize, char)), Box<dyn Error>> {
        let mut num_str = String::new();
        let mut dot_count = 0;
        let mut pos: usize = pos;
        let mut curr_char: Option<char> = Self::set_current_char(text, pos);

        while let Some(ch) = curr_char {
            if ch == '.' {
                dot_count += 1;
                if dot_count > 1 {
                    break;
                }
            } else if !ch.is_numeric() {
                break;
            }
            num_str.push(ch);
            pos += 1;
            curr_char = Self::set_current_char(text, pos);
        }

        curr_char = Self::set_current_char(text, pos - 1);

        if dot_count == 0 {
            match num_str.parse() {
                Ok(val) => Ok((Token::Int(val), (num_str.len(), curr_char.unwrap()))),
                Err(err) => Err(Box::new(err)),
            }
        } else {
            match num_str.parse() {
                Ok(val) => Ok((Token::Float(val), (num_str.len(), curr_char.unwrap()))),
                Err(err) => Err(Box::new(err)),
            }
        }
    }
}