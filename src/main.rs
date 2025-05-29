use designtime_rs::{
    OpCode, VM, ir::Program, lexer::Lexer, parser::Parser, validate_and_load_workspace,
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

    let config = validate_and_load_workspace();
    println!("Project loaded: {:?}", config.project.name);

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

    fn compile_example() -> Vec<OpCode> {
        vec![
            OpCode::Const(1.0),
            OpCode::Const(2.0),
            OpCode::Add,
            OpCode::Const(3.0),
            OpCode::Const(4.0),
            OpCode::Sub,
            OpCode::Mul,
        ]
    }

    let bytecode = compile_example();

    let mut vm = VM::new();
    match vm.run(&bytecode) {
        Ok(result) => println!("=== Result ===\n{}", result),
        Err(e) => eprintln!("Runtime error: {}", e),
    }
}
