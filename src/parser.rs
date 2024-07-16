use crate::types::Token;


// 'c lifetime stands for the slice of characters of each token
// 't lifetime stands for the slice of tokens
pub struct Parser <'t, 'c> {
    raw_tokens: &'t [&'c [char]]
}

impl <'t, 'c> Parser<'t,'c> {
    pub fn new(raw_tokens: &'t [&'c [char]]) -> Self {
        Self { raw_tokens }
    }

    fn deque(&mut self) {
        self.raw_tokens = &self.raw_tokens[1..];
    }

    fn next_token(&mut self) -> Option<Token<'c>> {
        if self.raw_tokens.len() == 0 {
            return None
        }

        None
    }
}

impl <'t, 'c> Iterator for Parser<'t, 'c> {
    type Item = Token<'c>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
