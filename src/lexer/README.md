# Chapter 1: The Lexer


## Overview

The lexer is the first vital step. It takes in the source code and breaks it down into tokens. These tokens are then used by the parser to build an abstract syntax tree (AST).

### Usage:
```rust
fn main() {
    let source = "<div>Hello world</div>";
    let mut lexer = Lexer::new(source);

    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token.kind == TokenKind::EOF {
            break;
        }
    }
}
```
    