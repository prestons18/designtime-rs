use crate::ast::{ASTNode, Function, Node, PageDecl};
use crate::ir::function::Instruction;
use crate::ir::{function::IrFunction, ir_node::IrNode, page::IrPage};

fn ir_node_from_ast(node: &Node) -> IrNode {
    match node {
        Node::Text(text) => IrNode::Text(text.clone()),

        Node::Expr(expr) => IrNode::Expr(expr.clone()),

        Node::Element {
            name,
            attrs,
            children,
        } => {
            let ir_attrs = attrs
                .iter()
                .map(|a| (a.name.clone(), a.value.clone()))
                .collect();

            let ir_children = children.iter().map(ir_node_from_ast).collect();

            IrNode::Element {
                name: name.clone(),
                attrs: ir_attrs,
                children: ir_children,
            }
        }

        Node::Fragment(children) => {
            let ir_children = children.iter().map(ir_node_from_ast).collect();

            IrNode::Fragment(ir_children)
        }
    }
}

fn ir_function_from_ast(func: &Function) -> IrFunction {
    // Convert params from Vec<String> to Vec<&String> for IrFunction::new
    let param_refs: Vec<&String> = func.params.iter().collect();

    // Replace this with actual bytecode compilation logic
    let instructions = vec![
        Instruction::LoadParam(0),
        Instruction::LoadConst("10".to_string()),
        Instruction::CallFunction("add".to_string()),
        Instruction::Return,
    ];

    IrFunction::new(&func.name, param_refs, instructions)
}

fn ir_page_from_ast(page: &PageDecl) -> IrPage {
    let ir_render_nodes = page.render.iter().map(ir_node_from_ast).collect();

    let ir_functions = page.functions.iter().map(ir_function_from_ast).collect();

    // Pass layout as Option<&String> using .as_ref()
    IrPage::new(
        &page.name,
        page.layout.as_ref(),
        ir_render_nodes,
        ir_functions,
    )
}

pub fn compile_ast_to_ir(ast_nodes: &[ASTNode]) -> Vec<IrPage> {
    ast_nodes
        .iter()
        .filter_map(|node| {
            if let ASTNode::Page(page_decl) = node {
                Some(ir_page_from_ast(page_decl))
            } else {
                None
            }
        })
        .collect()
}
