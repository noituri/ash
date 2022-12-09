pub struct Scope<T: Default>(Vec<T>);

impl<T: Default> Scope<T> {
    pub fn new() -> Self {
        Self(vec![Default::default()])
    }

    pub fn enter(&mut self) {
        self.0.push(Default::default())
    }

    pub fn leave(&mut self) -> T {
        self.0.remove(self.0.len()-1)
    }

    pub fn current(&self) -> &T {
        &self.0[self.0.len()-1]
    }

    pub fn current_mut(&mut self) -> &mut T {
        self.0.last_mut().unwrap()
    }
}