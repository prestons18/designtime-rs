#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Node {
    Element {
        tag_name: String,
        attributes: Vec<(String, String)>,
        class_names: Vec<String>,
        children: Vec<Node>,
    },
    Text(String),
}
