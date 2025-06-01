# Chapter 2: The Parser

The parser takes the tokens from the lexer and builds an abstract syntax tree (AST).

### Usage:
```rust
fn main() {
    let source = "<div>Hello world</div>";
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    match parser.parse() {
        Ok(node) => println!("{:#?}", node),
        Err(e) => eprintln!("Parse error: {}", e),
    }
}
```