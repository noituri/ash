use std::collections::HashMap;

use crate::ty::Ty;

use super::{Env, Id};

pub struct Context {
    env: Env,
    locals: HashMap<Id, Local>,
}

#[derive(Debug)]
struct Local {
    id: Id,
    ty: Option<Ty>,
    points_to: Option<Id>,
    depth: usize, // TODO: Remove later?
}

impl Context {
    pub fn new() -> Self {
        // TODO: Define globals
        // TODO: Desugar 'last expression returns value'
        let env = Env::default();
        let locals = HashMap::from_iter([(
            Id::new(0),
            Local {
                id: Id::new(0),
                ty: None,
                points_to: None,
                depth: 0,
            },
        )]);
        Self { env, locals }
    }

    pub fn get_env(&self) -> &Env {
        &self.env
    }

    pub fn set_env(&mut self, new_env: Env) {
        self.env = new_env;
    }

    pub(crate) fn var_type_at(&self, id: Id) -> Ty {
        dbg!(self.locals.get(&id));
        let local = self.locals.get(&id).unwrap();
        match local.ty.clone() {
            Some(ty) => ty,
            None => {
                let points_to = local.points_to.unwrap();
                if id != points_to {
                    self.var_type_at(points_to)
                } else {
                    unreachable!()
                }
            }
        }
    }

    pub(crate) fn new_var(&mut self, id: Id, ty: Ty) {
        self.locals.insert(id, Local {
            id,
            ty: Some(ty),
            depth: 0,
            points_to: None,
        });
    }

    // TODO: storing depth might not be needed at this stage
    pub(crate) fn resolve(&mut self, id: Id, depth: usize, ty: Option<Ty>, points_to: Id) {
        self.locals.insert(id, Local {
            id,
            depth,
            ty,
            points_to: Some(points_to)
        });
    }
}
