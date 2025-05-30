use super::function::IrFunction;
use super::page::IrPage;

/// Represents the entire program in IR form
#[derive(Debug, Clone)]
pub struct IrProgram {
    pub pages: Vec<IrPage>,
    pub functions: Vec<IrFunction>,
}

impl IrProgram {
    pub fn new(pages: Vec<IrPage>, functions: Vec<IrFunction>) -> Self {
        Self { pages, functions }
    }

    /// Find a page by name
    pub fn get_page(&self, name: &str) -> Option<&IrPage> {
        self.pages.iter().find(|page| page.name == name)
    }

    /// Find a function by name
    pub fn get_function(&self, name: &str) -> Option<&IrFunction> {
        self.functions.iter().find(|func| func.name == name)
    }
}
