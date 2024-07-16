pub enum TokenType {
    Keyword,
    Identifier,
    Operator,
    Literal,
    Number,
    Delimiter,
    Mismatch,
}

pub struct Token<'c> {
    token_type: TokenType,
    value: &'c [char]
}

impl From<TokenType> for String {

    fn from(value: TokenType) -> Self {
        match value {
            TokenType::Keyword => r"\b(WITH|AS|SELECT|FROM|WHERE|INNER|OUTER|LEFT|RIGHT|JOIN|)\b".to_owned(),
            TokenType::Identifier => todo!(),
            TokenType::Operator => todo!(),
            TokenType::Literal => todo!(),
            TokenType::Number => todo!(),
            TokenType::Delimiter => todo!(),
            TokenType::Mismatch => todo!(),
        }
    }

}

// RASCUNHO
pub enum DqlNode {
    Select(Vec<String>),
    From(String),
    Where(String),
    Join(String, Box<DqlNode>),
    Union(Vec<DqlNode>)
}