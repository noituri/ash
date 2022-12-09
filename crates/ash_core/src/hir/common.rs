use std::collections::{HashSet, VecDeque};

use crate::{core::{Spanned, Context}, parser::Stmt};


// TODO: Figure out better way of doing this
// It makes sure variables are initialized in such order that
// the variables, the initializer depends on, are available at the time.
// The current implementation of VM needs to know values of every variable that
// is needed to initialize declared variable
pub(crate) fn sort_root(ctx: &Context, ast: Vec<Spanned<Stmt>>) -> Vec<Spanned<Stmt>> {
    let mut sorted_ast = Vec::new();
    let mut postponed = Vec::new();
    let mut declared = HashSet::new();
    let mut unsorted_vars = VecDeque::new();
    for stmt in ast {
        match stmt.0 {
            Stmt::VariableDecl { id, .. } => {
                let deps = ctx.get_var_deps(id);
                if deps.is_empty() {
                    sorted_ast.push(stmt);
                    declared.insert(id);
                } else {
                    for dep in deps {
                        if !declared.contains(dep) {
                            unsorted_vars.push_back((id, stmt, deps));
                            break;
                        }
                    }
                }
            }
            _ => postponed.push(stmt),
        }
    }

    // TODO: Make sure it doesn't loop forever
    while let Some((id, v, deps)) = unsorted_vars.pop_front() {
        let mut resolved = true;
        for dep in deps {
            if !declared.contains(dep) {
                resolved = false;
                unsorted_vars.push_back((id, v.clone(), deps));
                break;
            }
        }

        if resolved {
            declared.insert(id);
            sorted_ast.push(v);
        }
    }

    sorted_ast.append(&mut postponed);
    sorted_ast
}