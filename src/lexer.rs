#[derive(Debug)]
pub struct Lexer<'c> {
    content: &'c [char]
}

impl <'c> Lexer<'c> {
    pub fn new(content: &'c [char]) -> Self {
        Self { content }
    }

    // trim_left will walk white spaces until it finds something different
    // "             WHERE X IS NULL;" -> "WHERE X IS NULL;"
    fn trim_left(&mut self) {
        while self.content.len() > 0 && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }
    }

    // chop will receive a usize "n", read it from [0..n], and return the possible token
    // "SELECT *" -[chop(6)]-> "SELECT"
    fn chop(&mut self, n: usize) -> &'c [char] {
        let token = &self.content[0..n];
        self.content = &self.content[n..];
        token
    }

    // while given a condition, it will chop the next token
    // Ex: chope_while(|c| c.is_alphanumeric())
    fn chope_while<P: FnMut(&char) -> bool> (&mut self, mut predicate: P) -> &'c [char] {
        let mut n = 0;
        while n < self.content.len() && predicate(&self.content[n]) {
            n += 1
        }

        return self.chop(n);
    }

    fn next_token(&mut self) -> Option<&'c [char]> {
        self.trim_left();

        if self.content.len() == 0 {
            return None
        }

        // 123th -[mapped]-> 123
        if self.content[0].is_numeric() {
            return Some(self.chope_while(|c| c.is_numeric()));
        }

        // Life_42 -[mapped]-> Life_42. TODO: check for all cases here
        if self.content[0].is_alphabetic() {
            return Some(self.chope_while(|c| c.is_alphanumeric() || *c == '_'));
        }

        // Here, we need to catch all non aphabetic symbols
        // catching ||, | |, ::, <=, < =, >, >=, > =
        return Some(self.chope_while(|c| !c.is_alphanumeric() || c.is_whitespace()))
    }
}

impl<'c> Iterator for Lexer<'c> {
    type Item = &'c [char];

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;

    #[test]
    fn select_star(){
        let query = "SELECT * FROM TABLE_NAME;";
        let query_chars: Vec<char> = query.chars().collect();
        let tokens: Vec<&[char]> = Lexer::new(&query_chars).collect();

        let owned_tokens: Vec<String> = tokens.into_iter().map(|chars| chars.iter().collect()).collect();
        let trimmed_tokens: Vec<&str> = owned_tokens.iter().map(|c| c.trim()).collect();

        assert_eq!(trimmed_tokens, vec!["SELECT", "*", "FROM", "TABLE_NAME", ";"])
    }

    #[test]
    fn simple_select(){
        let query = "SELECT COLUMN_A, COLUMN_B FROM TABLE_NAME;";
        let query_chars: Vec<char> = query.chars().collect();
        let tokens: Vec<&[char]> = Lexer::new(&query_chars).collect();

        let owned_tokens: Vec<String> = tokens.into_iter().map(|chars| chars.iter().collect()).collect();
        let trimmed_tokens: Vec<&str> = owned_tokens.iter().map(|c| c.trim()).collect();

        assert_eq!(trimmed_tokens, vec!["SELECT", "COLUMN_A", ",", "COLUMN_B", "FROM", "TABLE_NAME", ";",])
    }

    #[test]
    fn simple_where(){
        let query = "SELECT COLUMN_A, COLUMN_B FROM TABLE_NAME WHERE COLUMN_A >= 42;";
        let query_chars: Vec<char> = query.chars().collect();
        let tokens: Vec<&[char]> = Lexer::new(&query_chars).collect();

        let owned_tokens: Vec<String> = tokens.into_iter().map(|chars| chars.iter().collect()).collect();
        let trimmed_tokens: Vec<&str> = owned_tokens.iter().map(|c| c.trim()).collect();

        assert_eq!(trimmed_tokens, vec!["SELECT", "COLUMN_A", ",", "COLUMN_B", "FROM", "TABLE_NAME", "WHERE", "COLUMN_A", ">=", "42", ";"])
    }

    #[test]
    fn simple_where_spaced_operators(){
        let query = "SELECT COLUMN_A, COLUMN_B FROM TABLE_NAME WHERE COLUMN_A > = 42;";
        let query_chars: Vec<char> = query.chars().collect();
        let tokens: Vec<&[char]> = Lexer::new(&query_chars).collect();

        let owned_tokens: Vec<String> = tokens.into_iter().map(|chars| chars.iter().collect()).collect();
        let trimmed_tokens: Vec<&str> = owned_tokens.iter().map(|c| c.trim()).collect();

        assert_eq!(trimmed_tokens, vec!["SELECT", "COLUMN_A", ",", "COLUMN_B", "FROM", "TABLE_NAME", "WHERE", "COLUMN_A", "> =", "42", ";",])
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
        let query_chars: Vec<char> = query.chars().collect();
        let tokens: Vec<&[char]> = Lexer::new(&query_chars).collect();

        let owned_tokens: Vec<String> = tokens.into_iter().map(|chars| chars.iter().collect()).collect();
        let trimmed_tokens: Vec<&str> = owned_tokens.iter().map(|c| c.trim()).collect();

        assert_eq!(trimmed_tokens, vec![
            "WITH", "CTE", "AS", "(", "SELECT", "COLUMN_A", ",", "COLUMN_B", ",", "COLUMN_C", "FROM", "TABLE_NAME", ")",
            "SELECT", "COLUMN_A", ",", "COLUMN_B", "FROM", "CTE", "WHERE", "COLUMN_A", "> =", "42", ";"
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
        let query_chars: Vec<char> = query.chars().collect();
        let tokens: Vec<&[char]> = Lexer::new(&query_chars).collect();

        let owned_tokens: Vec<String> = tokens.into_iter().map(|chars| chars.iter().collect()).collect();
        let trimmed_tokens: Vec<&str> = owned_tokens.iter().map(|c| c.trim()).collect();

        assert_eq!(trimmed_tokens, vec![
            "SELECT", "COLUMN_A", ",", "COLUMN_B", "FROM",
            "(", "SELECT", "COLUMN_A", ",", "COLUMN_B", ",", "COLUMN_C", "FROM", "TABLE_NAME", "WHERE", "COLUMN_B", "<=", "42", ")",
            "WHERE", "COLUMN_A", "> =", "42", ";"
        ])
    }

}