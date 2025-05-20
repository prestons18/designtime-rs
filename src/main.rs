mod lexer;
mod parser;
mod ast;
mod vm;
mod bytecode;
mod emitter_js;
mod emitter_py;

use lexer::tokenize;
use parser::parse;
use vm::{Runtime, StdOut};
use bytecode::compile_ast;
use emitter_js::emit_js;
use emitter_py::emit_python;

fn main() {
    let input = r#"print("hello world")"#;

    // Step 1: Tokenize & Parse
    let tokens = tokenize(input);
    let ast = parse(&tokens);

    // Step 2: Compile to Bytecode
    let bytecode = compile_ast(&ast);

    // Step 3: Run in VM
    let mut stdout = StdOut;
    let mut runtime = Runtime::new(&mut stdout);
    runtime.run_bytecode(&bytecode);

    // Step 4: Emit to JS and Python
    let js_output = emit_js(&bytecode);
    println!("Generated JavaScript:\n{}", js_output);

    let py_output = emit_python(&bytecode);
    println!("Generated Python:\n{}", py_output);
}
