use crate::ast::{ASTNode, Function, Node, PageDecl};
use crate::ir::function::Instruction;
use crate::ir::{function::IrFunction, ir_node::IrNode, page::IrPage};

fn ir_node_from_ast(node: &Node) -> IrNode {
    match node {
        // Handle text nodes - skip empty ones
        Node::Text(text) => {
            let trimmed = text.trim();
            if trimmed.is_empty() {
                return IrNode::Fragment(Vec::new());
            }
            IrNode::Text(trimmed.to_string())
        },

        // Handle expressions
        Node::Expr(expr) => IrNode::Expr(expr.clone()),

        // Handle elements
        Node::Element {
            name,
            attrs,
            children,
        } => {
            // Convert attributes
            let ir_attrs: Vec<_> = attrs
                .iter()
                .map(|a| (a.name.clone(), a.value.clone()))
                .collect();

            // Process children recursively and flatten fragments
            let mut ir_children = Vec::new();
            for child in children {
                let processed = ir_node_from_ast(child);
                match processed {
                    // Flatten fragments into parent
                    IrNode::Fragment(nodes) => ir_children.extend(nodes),
                    // Keep other nodes as is
                    node => ir_children.push(node),
                }
            }


            IrNode::Element {
                name: name.clone(),
                attrs: ir_attrs,
                children: ir_children,
            }
        }


        // Handle fragments - this case might not be needed if we're flattening fragments above
        Node::Fragment(children) => {
            let mut nodes = Vec::new();
            for child in children {
                match ir_node_from_ast(child) {
                    IrNode::Fragment(inner_nodes) => nodes.extend(inner_nodes),
                    node => nodes.push(node),
                }
            }
            IrNode::Fragment(nodes)
        }
    }
}

fn ir_function_from_ast(func: &Function) -> IrFunction {
    let param_refs: Vec<&String> = func.params.iter().collect();

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
