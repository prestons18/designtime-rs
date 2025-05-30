#[cfg(test)]
mod tests {
    use crate::ir::function::{Instruction, IrFunction};

    #[test]
    fn test_ir_function_creation() {
        let instructions = vec![
            Instruction::LoadParam(0),
            Instruction::LoadConst("10".to_string()),
            Instruction::CallFunction("add".to_string()),
            Instruction::Return,
        ];

        let func = IrFunction::new("myFunc", vec!["x"], instructions.clone());

        assert_eq!(func.name, "myFunc");
        assert_eq!(func.params, vec!["x"]);
        assert_eq!(func.instructions.len(), 4);

        // Check individual instructions match
        match &func.instructions[0] {
            Instruction::LoadParam(idx) => assert_eq!(*idx, 0),
            _ => panic!("Expected LoadParam"),
        }

        match &func.instructions[1] {
            Instruction::LoadConst(val) => assert_eq!(val, "10"),
            _ => panic!("Expected LoadConst"),
        }

        match &func.instructions[2] {
            Instruction::CallFunction(name) => assert_eq!(name, "add"),
            _ => panic!("Expected CallFunction"),
        }

        match &func.instructions[3] {
            Instruction::Return => {} // OK
            _ => panic!("Expected Return"),
        }
    }
}
