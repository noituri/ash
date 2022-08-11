use std::{cell::RefCell, collections::HashMap, rc::Rc};

use chumsky::prelude::Simple;

use crate::ty::Value;

use super::{AshResult, Spanned};

#[derive(Debug, Clone, Default)]
pub struct Env(Rc<RefCell<InnerEnv>>);

impl Env {
    pub fn new_inner(&self) -> Self {
        Self(Rc::new(RefCell::new(InnerEnv {
            outer: Some(self.clone()),
            ..Default::default()
        })))
    }

    pub fn outer(&self) -> Self {
        self.0.take().outer.unwrap()
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.0.borrow_mut().variables.insert(name, value);
    }

    pub fn assign(&mut self, (name, span): Spanned<String>, value: Value) -> AshResult<(), String> {
        let mut inner = self.0.borrow_mut();
        let var = inner.variables.get_mut(&name);
        match var {
            Some(var) => *var = value,
            None => match inner.outer {
                Some(ref mut outer) => outer.assign((name, span), value)?,
                None => {
                    return Err(vec![Simple::custom(
                        span,
                        format!("Undefined variable: {name}"),
                    )])
                }
            },
        }

        Ok(())
    }

    pub fn assign_at(&mut self, name: Spanned<String>, value: Value, distance: usize) {
        self.ancestor(distance).assign(name, value).unwrap()
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        let inner = self.0.borrow();
        let value = inner.variables.get(name).cloned();
        if value.is_none() {
            if let Some(outer) = &inner.outer {
                return outer.get(name);
            }
        }

        value
    }

    pub fn get_at(&self, name: &str, distance: usize) -> Value {
        self.ancestor(distance).get(name).unwrap()
    }

    pub fn names(&self) -> Vec<String> {
        self.0.borrow().variables.keys().cloned().collect()
    }

    fn ancestor(&self, distance: usize) -> Self {
        let mut env = self.clone();
        for _ in 0..distance {
            let outer = env.0.borrow().outer.as_ref().unwrap().clone();
            env = outer;
        }

        env
    }
}

#[derive(Debug, Default)]
pub struct InnerEnv {
    pub outer: Option<Env>,
    variables: HashMap<String, Value>,
}
