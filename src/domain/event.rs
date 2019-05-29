use std::collections::HashMap;

pub struct Event {
    pub name: String,
    pub params: HashMap<String, Value>,
}

pub enum Value {
    String(String),
    Int(i64),
    Float(f64),
}