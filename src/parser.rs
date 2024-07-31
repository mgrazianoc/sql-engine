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

    fn deque(&mut self) -> &'c [char] {
        let raw_token = &self.raw_tokens[0];
        self.raw_tokens = &self.raw_tokens[1..];
        raw_token
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.raw_tokens.len() == 0 {
            return None
        }

        let raw_token = self.deque();
        let token: Token = raw_token.into();

        Some(token)
    }
}

impl <'t, 'c> Iterator for Parser<'t, 'c> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::{lexer::Lexer, types::{Token, TokenType}};

    use super::Parser;

    #[test]
    fn select_star(){
        let query = "SELECT * FROM TABLE_NAME;";
        let query_chars: Vec<char> = query.chars().collect();
        let raw_tokens: Vec<&[char]> = Lexer::new(&query_chars).collect();

        let tokens: Vec<Token> = Parser::new(&raw_tokens).collect();
        assert_eq!(tokens, vec![
            Token { token_type: TokenType::Keyword, value: "SELECT".to_string() },
            Token { token_type: TokenType::Asterisk, value: "*".to_string() },
            Token { token_type: TokenType::Keyword, value: "FROM".to_string() },
            Token { token_type: TokenType::Identifier, value: "TABLE_NAME".to_string() },
            Token { token_type: TokenType::Delimiter, value: ";".to_string() },
        ])
    }

    #[test]
    fn simple_select(){
        let query = "SELECT COLUMN_A, COLUMN_B FROM TABLE_NAME;";
        let query_chars: Vec<char> = query.chars().collect();
        let raw_tokens: Vec<&[char]> = Lexer::new(&query_chars).collect();

        let tokens: Vec<Token> = Parser::new(&raw_tokens).collect();
        assert_eq!(tokens, vec![
            Token { token_type: TokenType::Keyword, value: "SELECT".to_string() },
            Token { token_type: TokenType::Identifier, value: "COLUMN_A".to_string() },
            Token { token_type: TokenType::Delimiter, value: ",".to_string() },
            Token { token_type: TokenType::Identifier, value: "COLUMN_B".to_string() },
            Token { token_type: TokenType::Keyword, value: "FROM".to_string() },
            Token { token_type: TokenType::Identifier, value: "TABLE_NAME".to_string() },
            Token { token_type: TokenType::Delimiter, value: ";".to_string() },
        ])
    }

    #[test]
    fn simple_where(){
        let query = "SELECT COLUMN_A, COLUMN_B FROM TABLE_NAME WHERE COLUMN_A >= 42;";
        let query_chars: Vec<char> = query.chars().collect();
        let raw_tokens: Vec<&[char]> = Lexer::new(&query_chars).collect();

        let tokens: Vec<Token> = Parser::new(&raw_tokens).collect();
        assert_eq!(tokens, vec![
            Token { token_type: TokenType::Keyword, value: "SELECT".to_string() },
            Token { token_type: TokenType::Identifier, value: "COLUMN_A".to_string() },
            Token { token_type: TokenType::Delimiter, value: ",".to_string() },
            Token { token_type: TokenType::Identifier, value: "COLUMN_B".to_string() },
            Token { token_type: TokenType::Keyword, value: "FROM".to_string() },
            Token { token_type: TokenType::Identifier, value: "TABLE_NAME".to_string() },
            Token { token_type: TokenType::Keyword, value: "WHERE".to_string() },
            Token { token_type: TokenType::Identifier, value: "COLUMN_A".to_string() },
            Token { token_type: TokenType::Operator, value: ">=".to_string() },
            Token { token_type: TokenType::Number, value: "42".to_string() },
            Token { token_type: TokenType::Delimiter, value: ";".to_string() },
        ])
    }

    #[test]
    fn simple_where_spaced_operators(){
        let query = "SELECT COLUMN_A, COLUMN_B FROM TABLE_NAME WHERE COLUMN_A > = 42;";
        let query_chars: Vec<char> = query.chars().collect();
        let raw_tokens: Vec<&[char]> = Lexer::new(&query_chars).collect();

        let tokens: Vec<Token> = Parser::new(&raw_tokens).collect();
        assert_eq!(tokens, vec![
            Token { token_type: TokenType::Keyword, value: "SELECT".to_string() },
            Token { token_type: TokenType::Identifier, value: "COLUMN_A".to_string() },
            Token { token_type: TokenType::Delimiter, value: ",".to_string() },
            Token { token_type: TokenType::Identifier, value: "COLUMN_B".to_string() },
            Token { token_type: TokenType::Keyword, value: "FROM".to_string() },
            Token { token_type: TokenType::Identifier, value: "TABLE_NAME".to_string() },
            Token { token_type: TokenType::Keyword, value: "WHERE".to_string() },
            Token { token_type: TokenType::Identifier, value: "COLUMN_A".to_string() },
            Token { token_type: TokenType::Operator, value: ">=".to_string() },
            Token { token_type: TokenType::Number, value: "42".to_string() },
            Token { token_type: TokenType::Delimiter, value: ";".to_string() },
        ])
    }

    #[test]
    fn simple_cte(){
        let query = r#"
            WITH CTE AS (
                SELECT
                    COLUMN_A,
                    COLUMN_B,
                    COLUMN_C
                FROM TABLE_NAME
            )
            SELECT
                COLUMN_A,
                COLUMN_B
            FROM CTE
            WHERE COLUMN_A > = 42;
        "#;
        let re = Regex::new(r"[\n\t\r]").unwrap();
        let clean_query = re.replace_all(&query, "").to_string();

        let query_chars: Vec<char> = clean_query.chars().collect();
        let raw_tokens: Vec<&[char]> = Lexer::new(&query_chars).collect();

        let tokens: Vec<Token> = Parser::new(&raw_tokens).collect();
        assert_eq!(tokens, vec![
            Token { token_type: TokenType::Keyword, value: "WITH".to_string() },
            Token { token_type: TokenType::Identifier, value: "CTE".to_string() },
            Token { token_type: TokenType::Keyword, value: "AS".to_string() },
            Token { token_type: TokenType::Delimiter, value: "(".to_string() },
            Token { token_type: TokenType::Keyword, value: "SELECT".to_string() },
            Token { token_type: TokenType::Identifier, value: "COLUMN_A".to_string() },
            Token { token_type: TokenType::Delimiter, value: ",".to_string() },
            Token { token_type: TokenType::Identifier, value: "COLUMN_B".to_string() },
            Token { token_type: TokenType::Delimiter, value: ",".to_string() },
            Token { token_type: TokenType::Identifier, value: "COLUMN_C".to_string() },
            Token { token_type: TokenType::Keyword, value: "FROM".to_string() },
            Token { token_type: TokenType::Identifier, value: "TABLE_NAME".to_string() },
            Token { token_type: TokenType::Delimiter, value: ")".to_string() },
            Token { token_type: TokenType::Keyword, value: "SELECT".to_string() },
            Token { token_type: TokenType::Identifier, value: "COLUMN_A".to_string() },
            Token { token_type: TokenType::Delimiter, value: ",".to_string() },
            Token { token_type: TokenType::Identifier, value: "COLUMN_B".to_string() },
            Token { token_type: TokenType::Keyword, value: "FROM".to_string() },
            Token { token_type: TokenType::Identifier, value: "CTE".to_string() },
            Token { token_type: TokenType::Keyword, value: "WHERE".to_string() },
            Token { token_type: TokenType::Identifier, value: "COLUMN_A".to_string() },
            Token { token_type: TokenType::Operator, value: ">=".to_string() },
            Token { token_type: TokenType::Number, value: "42".to_string() },
            Token { token_type: TokenType::Delimiter, value: ";".to_string() },
        ])
    }

    #[test]
    fn simple_subquery(){
        let query = r#"
            SELECT
                COLUMN_A,
                COLUMN_B
            FROM (
                SELECT
                    COLUMN_A,
                    COLUMN_B,
                    COLUMN_C
                FROM TABLE_NAME
                WHERE COLUMN_B <= 42
            )
            WHERE COLUMN_A > = 42;
        "#;
        let re = Regex::new(r"[\n\t\r]").unwrap();
        let clean_query = re.replace_all(&query, "").to_string();

        let query_chars: Vec<char> = clean_query.chars().collect();
        let raw_tokens: Vec<&[char]> = Lexer::new(&query_chars).collect();

        let tokens: Vec<Token> = Parser::new(&raw_tokens).collect();
        assert_eq!(tokens, vec![
            Token { token_type: TokenType::Keyword, value: "SELECT".to_string() },
            Token { token_type: TokenType::Identifier, value: "COLUMN_A".to_string() },
            Token { token_type: TokenType::Delimiter, value: ",".to_string() },
            Token { token_type: TokenType::Identifier, value: "COLUMN_B".to_string() },
            Token { token_type: TokenType::Keyword, value: "FROM".to_string() },
            Token { token_type: TokenType::Delimiter, value: "(".to_string() },
            Token { token_type: TokenType::Keyword, value: "SELECT".to_string() },
            Token { token_type: TokenType::Identifier, value: "COLUMN_A".to_string() },
            Token { token_type: TokenType::Delimiter, value: ",".to_string() },
            Token { token_type: TokenType::Identifier, value: "COLUMN_B".to_string() },
            Token { token_type: TokenType::Delimiter, value: ",".to_string() },
            Token { token_type: TokenType::Identifier, value: "COLUMN_C".to_string() },
            Token { token_type: TokenType::Keyword, value: "FROM".to_string() },
            Token { token_type: TokenType::Identifier, value: "TABLE_NAME".to_string() },
            Token { token_type: TokenType::Keyword, value: "WHERE".to_string() },
            Token { token_type: TokenType::Identifier, value: "COLUMN_B".to_string() },
            Token { token_type: TokenType::Operator, value: "<=".to_string() },
            Token { token_type: TokenType::Number, value: "42".to_string() },
            Token { token_type: TokenType::Delimiter, value: ")".to_string() },
            Token { token_type: TokenType::Keyword, value: "WHERE".to_string() },
            Token { token_type: TokenType::Identifier, value: "COLUMN_A".to_string() },
            Token { token_type: TokenType::Operator, value: ">=".to_string() },
            Token { token_type: TokenType::Number, value: "42".to_string() },
            Token { token_type: TokenType::Delimiter, value: ";".to_string() },
        ])
    }

}