pub mod dom;
pub mod transform;
pub mod html_mac;

pub mod prelude {
    pub use crate::dom::DomNode;
    pub use crate::transform::{transform, get_css};
    pub use crate::html_mac::*;
}
