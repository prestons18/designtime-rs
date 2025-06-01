use designtime_rs::{lexer::TokenKind, Lexer};

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