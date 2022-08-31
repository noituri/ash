use std::cell::Cell;

pub trait Collectable {}

pub struct GCObject<T> {
    inner: T,
    marked: Cell<bool>,
}
