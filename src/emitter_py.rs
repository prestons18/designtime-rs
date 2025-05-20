use crate::bytecode::Instruction;

pub fn emit_python(bytecode: &[Instruction]) -> String {
    bytecode.iter().map(|instr| match instr {
        Instruction::Print(msg) => format!("print({:?})", msg),
    }).collect::<Vec<_>>().join("\n")
}
