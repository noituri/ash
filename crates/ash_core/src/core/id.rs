use std::{cell::Cell, sync::Mutex};

use once_cell::sync::OnceCell;

// TODO: Remove global data
static ID_POOL: OnceCell<Mutex<IdPool>> = OnceCell::new();

pub(crate) fn next_id() -> Id {
    let pool = ID_POOL.get_or_init(|| Mutex::new(IdPool::new()));
    pool.lock().unwrap().next_id()
}

pub struct IdPool(Cell<usize>);

impl IdPool {
    pub const fn new() -> Self {
        Self(Cell::new(0))
    }

    pub(crate) fn next_id(&mut self) -> Id {
        self.0.set(self.0.get() + 1);
        Id::new(self.0.get())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) struct Id(usize);

impl Id {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}