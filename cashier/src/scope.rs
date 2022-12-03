use std::collections::HashMap;

use llvm_sys::prelude::{LLVMTypeRef, LLVMValueRef};

pub struct Scope<'a>(Vec<HashMap<&'a str, (LLVMValueRef, LLVMTypeRef)>>);

impl<'a> Scope<'a> {
    pub fn new() -> Self {
        Self(vec![HashMap::new()])
    }

    pub fn enter(&mut self) {
        self.0.push(HashMap::new())
    }

    pub fn set_var(&mut self, name: &'a str, value: (LLVMValueRef, LLVMTypeRef)) {
        let mut i = self.0.len() - 1;
        while self.0[i].get(name).is_none() {
            if i == 0 {
                i = self.0.len() - 1;
                break;
            }
            i -= 1;
        }
        self.0[i].insert(name, value);
    }

    pub fn get_var(&self, name: &'a str) -> (LLVMValueRef, LLVMTypeRef) {
        let mut i = self.0.len() - 1;
        while self.0[i].get(name).is_none() {
            i -= 1;
        }
        *self.0[i].get(name).unwrap()
    }

    pub fn leave(&mut self) {
        self.0.remove(self.0.len() - 1);
    }
}
