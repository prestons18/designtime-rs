#[derive(Debug, Clone)]
pub enum Node {
    Element {
        tag_name: String,
        class_names: Vec<String>,
        children: Vec<Node>,
    },
    Text(String),
}
