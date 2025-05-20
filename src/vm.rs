use crate::bytecode::Instruction;

pub trait OutputTarget {
    fn write(&mut self, message: &str);
}

pub struct StdOut;

impl OutputTarget for StdOut {
    fn write(&mut self, message: &str) {
        println!("{}", message);
    }
}

pub struct Runtime<'a> {
    pub output: &'a mut dyn OutputTarget,
}

impl<'a> Runtime<'a> {
    pub fn new(output: &'a mut dyn OutputTarget) -> Self {
        Self { output }
    }

    pub fn run_bytecode(&mut self, bytecode: &[Instruction]) {
        for instr in bytecode {
            match instr {
                Instruction::Print(msg) => self.output.write(msg),
            }
        }
    }
}
