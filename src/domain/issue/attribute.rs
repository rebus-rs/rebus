use std::rc::Rc;
use std::collections::HashMap;

pub struct AttributeKey(pub Rc<String>);

pub struct AttributeKeyStorage {
    keys: Vec<AttributeKey>,
    aliases: HashMap<AttributeKey, AttributeKey>,
}