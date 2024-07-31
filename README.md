# About
This is personal project for studying how one can construct a SQL engine, while also practicing rust.

## TODO

- [X] `Lexer`, reponsible for extracting `raw_tokens` from a query;
- [X] `Parser`, responsible for categorizing `raw_tokens` into valid `Tokens`;
- [ ] `DqlTreeBuilder`, responsible for structing the sequence of `Tokens` into `DQLNode`s;
- [ ] `Interpreter`, responsible for validating the query;
- [ ] `DqlTreeTransformer`, responsible for spanning options for manipulating valid DqlTree into others DqlTree;
- [ ] `Optimizer`, responsible for some optimizations in the query;

## Lexer Examples

```rs
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
```

## Parser Examples
```rs
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
```