use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Keyword,
    Identifier,
    Operator,
    Literal,
    Number,
    Delimiter,
    Asterisk,
    Mismatch,
}

// priority goes from up to bottom
const TOKENS_SPECS: &'static [(TokenType, &'static str)] = &[
    (TokenType::Keyword, r"\b(ALL|AND|ANY|AS|ASC|BETWEEN|CASE|DESC|DISTINCT|EXISTS|FROM|FULL|OUTER|GROUP|BY|HAVING|IN|INNER|IS|JOIN|LEFT|LIKE|LIMIT|NOT|NULL|OR|ORDER|RIGHT|ROWNUM|SELECT|TOP|UNION|WHERE|WITH)\b"),
    (TokenType::Identifier, r"[a-zA-Z_][a-zA-Z0-9_]*"),
    (TokenType::Operator, r"[<>]=|<>|=|<|>"),
    (TokenType::Literal, r"'[^']*'"),
    (TokenType::Number, r"\b\d+\b"),
    (TokenType::Delimiter, r"[;,()]"),
    (TokenType::Asterisk, r"\*"),
    (TokenType::Mismatch, r"."),
];

impl<'c> From<&'c [char]> for Token {

    fn from(value: &'c [char]) -> Self {
        let raw_token: String = value.iter().collect();

        let mut token = Token {
            token_type: TokenType::Mismatch,
            value: raw_token.replace(" ", "").to_owned()
        };

        for token_spec in TOKENS_SPECS {
            // we assume the regexs are all correct, otherwise we will leak regex internal abstractions
            let re = Regex::new(token_spec.1).unwrap();

            if Regex::is_match(&re, &raw_token) {
                token.token_type = token_spec.0.clone();
                break
            }
        }

        token
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String
}

// RASCUNHO
pub enum DqlNode {
    Select(Vec<Token>),
    From(Token),
    Join(Token, Box<DqlNode>),
    Where(Vec<Token>),
    GroupBy(Vec<Token>),
    Having(Vec<Token>),
    Union(Vec<DqlNode>),
    UnionAll(Vec<DqlNode>)
}