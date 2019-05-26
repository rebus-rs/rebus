use std::collections::HashMap;
pub use self::attribute::*;
pub use self::content::*;

pub mod attribute;
pub mod content;

struct Issue {
    id: AttributeKey,
    attrs: HashMap<AttributeKey, String>,
    content: Option<Box<dyn ContentProvider>>,
}