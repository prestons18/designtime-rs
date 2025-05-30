/// Represents an import declaration in the IR
#[derive(Debug, Clone)]
pub struct IrImport {
    /// The module name or path being imported
    pub module: String,
    /// The specific names imported from the module
    pub names: Vec<String>,
}

impl IrImport {
    pub fn new<S: Into<String>>(module: S, names: Vec<S>) -> Self {
        Self {
            module: module.into(),
            names: names.into_iter().map(Into::into).collect(),
        }
    }
}
