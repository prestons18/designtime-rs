use designtime_rs::{
    lexer::Lexer,
    parser::Parser,
    ir::Program
};

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

    // Step 1: Tokenize
    let tokens = Lexer::new(input).tokenize();
    println!("\n=== Tokens ===");
    println!("{:#?}", tokens);
    
    // Step 2: Parse tokens into AST
    let ast = Parser::new(tokens).parse();
    println!("\n=== AST ===");
    println!("{:#?}", ast);

    // Step 3: Compile to IR
    let module = Program::from_ast(ast);

    println!("\n=== IR ===");
    println!("{:#?}", module);

    // // Step 4: Generate JavaScript output using the React emitter
    // let mut emitter = create_emitter(TargetPlatform::React);
    // let js_code = emitter.emit_module(&module);
    
    // println!("\n=== JavaScript ===");
    // println!("{}", js_code);
    
    // // Step 5: Write to output file
    // std::fs::write("output.js", &js_code).expect("Failed to write output file");
    // println!("\nOutput written to output.js");
}