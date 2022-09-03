use std::collections::HashMap;

use crate::{parser::Expr, ty::Ty};

use super::{Env, Id};

pub struct Context {
    env: Env,
    location: String,
    locals: HashMap<Id, Local>,
    var_nodes: HashMap<Id, VarNode>,
}

#[derive(Debug, Clone)]
pub(crate) struct VarNode {
    pub id: Id,
    pub name: String,
    pub value: Expr,
    pub deps: Vec<Id>,
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
            var_nodes: HashMap::new(),
        }
    }

    pub fn get_env(&self) -> &Env {
        &self.env
    }

    pub fn set_env(&mut self, new_env: Env) {
        self.env = new_env;
    }

    pub(crate) fn get_var_deps(&self, id: Id) -> &[Id] {
        &self.var_nodes.get(&id).unwrap().deps
    }

    pub(crate) fn get_pointer_var_node(&self, id: Id) -> &VarNode {
        let local = self.get_pointed_local(id);
        self.var_nodes.get(&local.id).unwrap()
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

    pub(crate) fn var_type_at(&self, id: Id) -> Option<Ty> {
        let local = self.locals.get(&id)?;
        if local.ty.is_none() {
            let points_to = local.points_to?;
            if id != points_to {
                return self.var_type_at(points_to);
            } else {
                unreachable!()
            }
        }

        local.ty.clone()
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

    pub(crate) fn check_circular_dep(&self, id: Id, points_to: Id) -> Vec<VarNode> {
        if let Some(var_decl) = self.var_nodes.get(&points_to) {
            for dep in var_decl.deps.iter() {
                if *dep == id || *dep == var_decl.id {
                    return vec![var_decl.clone()];
                }

                let mut deps_path = self.check_circular_dep(id, *dep);
                if !deps_path.is_empty() {
                    let mut new_path = vec![var_decl.clone()];
                    new_path.append(&mut deps_path);
                    return new_path;
                }
            }
        }

        Vec::new()
    }

    pub(crate) fn resolve_new_var(&mut self, id: Id, name: String, value: Expr, deps: Vec<Id>) {
        self.var_nodes.insert(
            id,
            VarNode {
                id,
                name: name.clone(),
                value,
                deps,
            },
        );
        self.locals.insert(
            id,
            Local {
                id,
                name: Some(name),
                ty: None,
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
