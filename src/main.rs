use designtime_rs::{Lexer, Parser};

fn main() {
    let source = "<div>Hello world</div>";
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    match parser.parse() {
        Ok(node) => println!("{:#?}", node),
        Err(e) => eprintln!("Parse error: {}", e),
    }
}
