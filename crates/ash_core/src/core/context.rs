use std::collections::HashMap;

use crate::ty::Ty;

use super::{Env, Id};

pub struct Context {
    env: Env,
    location: String,
    locals: HashMap<Id, Local>,
}

#[derive(Debug)]
pub(crate) struct Local {
    pub id: Id,
    pub name: Option<String>,
    pub ty: Option<Ty>,
    pub points_to: Option<Id>,
    pub depth: usize, // TODO: Remove later?
}

impl Context {
    pub fn new(location: String) -> Self {
        // TODO: Define globals
        // TODO: Desugar 'last expression returns value'
        let env = Env::default();
        let locals = HashMap::new();
        // let locals = HashMap::from_iter([(
        //     Id::new(0),
        //     Local {
        //         id: Id::new(0),
        //         ty: None,
        //         points_to: None,
        //         depth: 0,
        //     },
        // )]);
        Self {
            env,
            location,
            locals,
        }
    }

    pub fn get_env(&self) -> &Env {
        &self.env
    }

    pub fn set_env(&mut self, new_env: Env) {
        self.env = new_env;
    }

    pub(crate) fn get_pointed_local(&self, id: Id) -> &Local {
        let local = self.locals.get(&id).unwrap();
        self.get_local(local.points_to.unwrap())
    }

    pub(crate) fn get_local(&self, id: Id) -> &Local {
        self.locals.get(&id).unwrap()
    }

    pub(crate) fn get_local_mut(&mut self, id: Id) -> &mut Local {
        self.locals.get_mut(&id).unwrap()
    }

    pub(crate) fn var_type_at(&self, id: Id) -> Ty {
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

    pub(crate) fn new_var(&mut self, id: Id, name: String, ty: Ty) {
        self.locals.insert(
            id,
            Local {
                id,
                name: Some(name),
                ty: Some(ty),
                depth: 0,
                points_to: None,
            },
        );
    }

    // TODO: storing depth might not be needed at this stage
    // Since IR desugars most blocks the depth may be invalid
    pub(crate) fn resolve(&mut self, id: Id, depth: usize, ty: Option<Ty>, points_to: Id) {
        self.locals.insert(
            id,
            Local {
                id,
                depth,
                ty,
                name: None,
                points_to: Some(points_to),
            },
        );
    }

    pub fn location(&self) -> &str {
        &self.location
    }
}
