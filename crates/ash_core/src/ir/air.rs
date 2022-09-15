use std::{ffi::CString, fs::File, io::Write};

use chumsky::chain::Chain;
use serde::Serialize;

use crate::{
    core::Spanned,
    parser::operator::BinaryOp,
    ty::{self, function::ProtoFunction, Value},
};

use super::{ir, Expr, Stmt};

#[derive(Default, Serialize)]
pub struct Header {
    pub version: (u8, u8, u8),
    pub instructions: Vec<Inst>,
    pub strings: Vec<u8>,
    pub extra: Vec<Extra>,
}

impl Header {
    pub fn new(version: (u8, u8, u8)) -> Self {
        Self {
            version,
            ..Default::default()
        }
    }
}

#[derive(Serialize)]
pub enum Inst {
    None, // Serves as undefined / null / no value
    Fun { params_len: u8, body_len: u32 },
    Call { arg_len: u8 },
    Var,
    Sum,
    I32(i32),
    F64(f64),
    Bool(bool),
    String,
    Ret,
    VarDecl(Ty),
    Assign,
}

// TODO: Enforce 4 bytes
#[derive(Serialize)]
pub enum Extra {
    TypedField(Ty),
    Type(Ty),
}

#[derive(Serialize)]
pub enum Ty {
    String,
    I32,
    F64,
    Bool,
    Void,
}

pub(crate) struct Compiler {
    header: Header,
}

impl Compiler {
    pub fn new() -> Self {
        assert_eq!(std::mem::size_of::<Inst>(), 16);
        // assert_eq!(std::mem::size_of::<Data>(), 4);

        Self {
            header: Header::new((0, 1, 0)),
        }
    }

    pub fn run(mut self, ast: Vec<Spanned<Stmt>>) {
        self.compile_statements(ast);
        self.create_file();
    }

    fn create_file(&self) {
        let bytes = bincode::serialize(&self.header).unwrap();
        dbg!(&bytes);
        let mut f = File::create("test.cash").unwrap();
        f.write_all(&bytes).unwrap();
    }

    fn compile_statements(&mut self, statements: Vec<Spanned<Stmt>>) {
        for (stmt, _) in statements {
            self.compile_stmt(stmt);
        }
    }

    fn compile_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::ProtoFunction(proto) => self.compile_fun(proto, Vec::new()),
            Stmt::Function(inner) => self.compile_fun(inner.proto.0, inner.body.0),
            Stmt::Expression(expr, _) => self.compile_expr(expr),
            Stmt::Return(expr, _) => self.compile_ret(expr),
            Stmt::VariableDecl { name, ty, value, .. } => self.compile_var_decl(name, value, ty),
            Stmt::VariableAssign { name, value, .. } => self.compile_assign(name.0, value),
            _ => unimplemented!(),
        }
    }

    // Must add new instruction in every case
    fn compile_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Literal(value) => self.compile_constant(value),
            Expr::Binary {
                left, op, right, ..
            } => self.compile_bin(*left, op, *right),
            Expr::Call { callee, args, .. } => self.compile_call(*callee, args),
            Expr::Variable(_, name, _) => self.compile_var(name),
            _ => unimplemented!(),
        }
    }

    fn convert_ty(&self, old_ty: ty::Ty) -> Ty {
        match old_ty {
            ty::Ty::String => Ty::String,
            ty::Ty::Bool => Ty::Bool,
            ty::Ty::I32 => Ty::I32,
            ty::Ty::F64 => Ty::F64,
            ty::Ty::Void => Ty::Void,
            ty::Ty::Fun(_, _) => todo!(),
            ty::Ty::DeferTyCheck(_, _) => unreachable!(),
        }
    }

    fn compile_assign(&mut self, name: String, value: Expr) {
        self.add_inst(Inst::Assign);
        self.add_string(name);
        self.compile_expr(value);
    }

    fn compile_var_decl(&mut self, name: String, value: Expr, var_ty: ty::Ty) {
        let var_ty = self.convert_ty(var_ty);
        self.add_inst(Inst::VarDecl(var_ty));
        self.compile_expr(value);
        self.add_string(name);
    }

    fn compile_var(&mut self, name: String) {
        self.add_inst(Inst::Var);
        self.add_string(name);
    }

    fn compile_call(&mut self, callee: Expr, args: Vec<Expr>) {
       self.add_inst(Inst::Call { arg_len: args.len() as u8 });
       self.compile_expr(callee);
       for arg in args {
        self.compile_expr(arg);
       }
    }

    fn compile_ret(&mut self, expr: Option<Expr>) {
        self.add_inst(Inst::Ret);

        match expr {
            Some(expr) => self.compile_expr(expr),
            None => self.add_inst(Inst::None) 
        }
    }

    fn compile_bin(&mut self, left: Expr, op: BinaryOp, right: Expr) {
        let inst = match op {
            BinaryOp::Sum => Inst::Sum,
            _ => unimplemented!(),
        };
        self.add_inst(inst);
        self.compile_expr(left);
        self.compile_expr(right);
    }

    fn compile_constant(&mut self, value: Value) {
        let inst = match value {
            Value::String(v) => {
                self.add_string(v);
                Inst::String
            }
            Value::I32(v) => Inst::I32(v),
            Value::F64(v) => Inst::F64(v),
            Value::Bool(v) => Inst::Bool(v),
        };

        self.add_inst(inst);
    }

    fn compile_fun(&mut self, proto: ProtoFunction, body: Vec<Spanned<Stmt>>) {
        let params_len = proto.params.len();
        let body_len = body.len();
        if body_len > u32::MAX as usize {
            panic!("Body too long")
        }

        self.add_inst(Inst::Fun {
            params_len: params_len as u8,
            body_len: body_len as u32,
        });

        self.add_string(proto.name);
        for (_, name, ty) in proto.params {
            self.compile_field(name, ty);
        }

        let return_ty = self.convert_ty(proto.ty.fun_return_ty());
        self.add_data(Extra::Type(return_ty));

        for (stmt, _) in body {
            self.compile_stmt(stmt);
        }
    }

    fn compile_field(&mut self, name: String, field_ty: ty::Ty) {
        let field_ty = self.convert_ty(field_ty);
        self.add_data(Extra::TypedField(field_ty));
        self.add_string(name);
    }

    fn add_inst(&mut self, inst: Inst) {
        self.header.instructions.push(inst)
    }

    fn add_string(&mut self, s: String) {
        let s = CString::new(s).expect("Proper C string");
        let mut bytes = s.as_c_str().to_bytes_with_nul().to_vec();

        self.header.strings.append(&mut bytes);
    }

    fn add_data(&mut self, data: Extra) {
        self.header.extra.push(data);
    }
}
