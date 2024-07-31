use std::{error::Error, fmt};

use crate::types::Token;

pub enum DQLError {
    SyntaxError(SyntaxError)
}

#[derive(Debug)]
pub struct SyntaxError;

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SyntaxError")
    }
}

impl Error for SyntaxError {}

pub struct Interpreter <'t> {
    _tokens: &'t [Token]
}

impl<'t> Interpreter<'t> {
    pub fn new(tokens: &'t [Token]) -> Self {
        Self { _tokens: tokens }
    }

    pub fn validate(&self) -> Result<(), DQLError> {
        
        Ok(())
    }
}