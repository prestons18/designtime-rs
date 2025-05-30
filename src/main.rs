use designtime_rs::{Parser, lexer::Lexer, util::compile_ast_to_ir, validate_and_load_workspace};

fn main() {
    let input = r#"
    import { Checkbox } from "@designtime.core.ui.MUI"
    
    page Home {
        layout: Glassmorphism
        render: { 
            <div class="container">
                <h1>Welcome to DesignTime</h1>
                <Checkbox checked={true}>Do you see this? {1+1}</Checkbox>
            </div>
        }
        functions: {
            onSelect: () => {
                let x = 40;
                let y = 2;
                let result = x + y;
                return result;
            }
        }
    }
    "#;

    println!("=== Input ===\n{}", input);

    let config = validate_and_load_workspace();
    println!("Project loaded: {:?}", config.project.name);

    let tokens = Lexer::new(input).tokenize();
    println!("\n=== Tokens ===\n{:#?}", tokens);

    let ast = match Parser::new(tokens).parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Parse error: {}", e.message());
            return;
        }
    };
    println!("\n=== AST ===\n{:#?}", ast);

    // Now compile AST to IR
    let module = compile_ast_to_ir(&ast);

    // Then print or execute the IR
    println!("\n=== IR Program ===\n{:#?}", module);
}
