#[derive(Debug)]
pub enum OpCode {
    Const(f64),
    Add,
    Sub,
    Mul,
    Div,
}

pub struct VM {
    stack: Vec<f64>,
}

impl VM {
    pub fn new() -> Self {
        VM { stack: Vec::new() }
    }

    pub fn run(&mut self, bytecode: &[OpCode]) -> Result<f64, String> {
        for instr in bytecode {
            match instr {
                OpCode::Const(val) => self.stack.push(*val),
                OpCode::Add => {
                    let b = self.stack.pop().ok_or("Stack underflow on Add")?;
                    let a = self.stack.pop().ok_or("Stack underflow on Add")?;
                    self.stack.push(a + b);
                }
                OpCode::Sub => {
                    let b = self.stack.pop().ok_or("Stack underflow on Sub")?;
                    let a = self.stack.pop().ok_or("Stack underflow on Sub")?;
                    self.stack.push(a - b);
                }
                OpCode::Mul => {
                    let b = self.stack.pop().ok_or("Stack underflow on Mul")?;
                    let a = self.stack.pop().ok_or("Stack underflow on Mul")?;
                    self.stack.push(a * b);
                }
                OpCode::Div => {
                    let b = self.stack.pop().ok_or("Stack underflow on Div")?;
                    if b == 0.0 {
                        return Err("Division by zero".into());
                    }
                    let a = self.stack.pop().ok_or("Stack underflow on Div")?;
                    self.stack.push(a / b);
                }
            }
        }
        self.stack.pop().ok_or("Stack empty after execution".into())
    }
}
