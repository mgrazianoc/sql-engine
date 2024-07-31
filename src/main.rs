pub mod lexer;
pub mod types;
pub mod parser;
pub mod dql_builder;
pub mod interpreter;

// let query = "SELECT ...";
// 1. Lexer(&query) -> [raw_tokens]
// 2. Parser([tokens]) -> [TOKENS]
// 3. DqlBuilder([TOKENS]) -> DqlTree
// 4. Interpreter(DqlTree) -> bool
// 5. Optimizer(DqlTree) -> DqlTree

fn main() {
    println!("Hello, world!");
}
