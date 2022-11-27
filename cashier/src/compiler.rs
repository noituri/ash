use ash_core::prelude::*;
use inkwell::{
    builder::Builder, context::Context, module::Module, passes::PassManager, values::FunctionValue,
};

pub struct Compiler<'a, 'c> {
    ctx: &'c Context,
    builder: Builder<'c>,
    module: Module<'c>,
    fpm: PassManager<FunctionValue<'c>>,
    src: &'a cash::Header,
    inst_offset: usize,
    str_offset: usize,
}

impl<'a, 'c> Compiler<'a, 'c> {
    pub fn new(
        src: &'a cash::Header,
        ctx: &'c Context,
        builder: Builder<'c>,
        module: Module<'c>,
        fpm: PassManager<FunctionValue<'c>>,
    ) -> Self {
        Self {
            ctx,
            src,
            fpm,
            builder,
            module,
            inst_offset: 0,
            str_offset: 0,
        }
    }

    pub fn compile(&mut self) {
        use cash::Inst;
        while let Some(inst) = self.read_inst() {
            match inst {
                Inst::Fun {
                    params_len: _,
                    body_len: _,
                } => {
                    let _name = self.read_string();
                }
                _ => todo!(),
            }
        }
    }

    fn read_inst(&mut self) -> Option<cash::Inst> {
        let inst = self.src.instructions.get(self.inst_offset).cloned();
        self.inst_offset += 1;

        inst
    }

    fn read_string(&mut self) -> String {
        let mut s = String::new();
        let mut offset = 0;
        for b in self.src.strings.iter().skip(self.str_offset) {
            s.push((*b).into());
            offset += 1;
        }

        self.str_offset += offset;
        s
    }
}
