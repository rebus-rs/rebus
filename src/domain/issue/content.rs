use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Description(pub Rc<String>);

pub struct ContentLog<'a> {
    provider: &'a dyn ContentProvider,
}

pub trait ContentProvider {
    fn description(&self) -> Option<Description>;
    fn log(&self) -> ContentLog;
}