use crate::bytecode::Instruction;

pub fn emit_js(bytecode: &[Instruction]) -> String {
    bytecode.iter().map(|instr| match instr {
        Instruction::Print(msg) => format!("console.log({:?});", msg),
    }).collect::<Vec<_>>().join("\n")
}
