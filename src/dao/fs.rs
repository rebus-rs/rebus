use std::cell::RefCell;
use crate::issue::{ContentProvider, Description, ContentLog};

pub struct FsContentProvider;

impl ContentProvider for FsContentProvider {
    fn description(&self) -> Option<Description> {
        unimplemented!()
    }

    fn log(&self) -> ContentLog {
        unimplemented!()
    }
}

pub struct LazyFsContentProvider {
    provider: FsContentProvider,
    description: RefCell<Option<Option<Description>>>,
}

impl ContentProvider for LazyFsContentProvider {
    fn description(&self) -> Option<Description> {
        if self.description.borrow().is_none() {
            *self.description.borrow_mut() = Some(self.provider.description());
        }
        self.description.borrow()
            .clone()
            .unwrap_or_default()
    }

    fn log(&self) -> ContentLog {
        unimplemented!()
    }
}