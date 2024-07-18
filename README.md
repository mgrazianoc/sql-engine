# About
This is personal project for studying how one can construct a SQL engine, while also practicing rust.

## TODO

- [X] `Lexer`, reponsible for extracting `raw_tokens` from a query;
- [X] `Parser`, responsible for categorizing `raw_tokens` into valid `Tokens`;
- [ ] `DqlTreeBuilder`, responsible for structing the sequence of `Tokens` into `DQLNode`s;
- [ ] `Interpreter`, responsible for validating the query;
- [ ] `DqlTreeTransformer`, responsible for spanning options for manipulating valid DqlTree into others DqlTree;
- [ ] `Optimizer`, responsible for some optimizations in the query;