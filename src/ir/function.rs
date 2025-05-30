#[derive(Debug, Clone)]
pub struct IrFunction {
    pub name: String,
    pub params: Vec<String>,
    pub instructions: Vec<Instruction>,
}

impl IrFunction {
    pub fn new<S: Into<String>>(name: S, params: Vec<S>, instructions: Vec<Instruction>) -> Self {
        Self {
            name: name.into(),
            params: params.into_iter().map(Into::into).collect(),
            instructions,
        }
    }
}

/// Instructions for the VM to execute inside functions.
#[derive(Debug, Clone)]
pub enum Instruction {
    LoadConst(String),    // Load a constant value (e.g. string, number)
    LoadParam(usize),     // Load function parameter by index
    CallFunction(String), // Call another function by name
    SetState(String),     // Set a state variable by name
    GetState(String),     // Get a state variable by name
    Return,               // Return from function
                          // Can extend with arithmetic, branches, etc.
}
