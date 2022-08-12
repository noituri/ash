use std::collections::HashMap;

use super::{Env, Id};

pub struct Context {
    env: Env,
    locals: HashMap<Id, usize>
}

impl Context {
    pub fn new() -> Self {
        let env = Env::default();
        let locals = HashMap::from_iter([(Id::new(0), 0)]);
        Self { env, locals }
    }

    pub fn get_env(&self) -> &Env {
        &self.env
    }

    pub fn set_env(&mut self, new_env: Env) {
        self.env = new_env;
    }

    pub(crate) fn resolve(&mut self, id: Id, depth: usize) {
        self.locals.insert(id, depth);
    }
}
