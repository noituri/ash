use std::{ffi::CString, fs::File, io::Write};

use chumsky::chain::Chain;
use serde::{Deserialize, Serialize};

use crate::{
    core::Spanned,
    parser::{operator::{BinaryOp, UnaryOp}, If},
    ty::{self, function::ProtoFunction, Value, Stmt, Expr},
};


#[derive(Default, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Inst {
    None, // Serves as undefined / null / no value
    Fun { params_len: u8, body_len: u32 },
    Call { arg_len: u8 },
    Block { len: u32 },
    Var,
    Sum,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
    LogicAnd,
    LogicOr,
    Not,
    Neg,
    I32(i32),
    F64(f64),
    Bool(bool),
    String,
    Ret,
    VarDecl(Ty),
    Assign,
    Loop { len: u32 },
    Repeat,
    Branch(u32, u32),
    Break,
}

// TODO: Enforce 4 bytes
#[derive(Serialize, Deserialize, Clone)]
pub enum Extra {
    Type(Ty),
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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

    // pub fn run(mut self, ast: Vec<Spanned<Stmt>>) -> Vec<u8> {
    //     self.compile_statements(ast);
    //     bincode::serialize(&self.header).unwrap()
    // }

    // fn compile_statements(&mut self, statements: Vec<Spanned<Stmt>>) {
    //     for (stmt, _) in statements {
    //         self.compile_stmt(stmt);
    //     }
    // }

    // fn compile_stmt(&mut self, stmt: Stmt) {
    //     match stmt {
    //         Stmt::Annotation(_, stmt) => self.compile_stmt(stmt.0), // TODO: Figure it out
    //         Stmt::ProtoFunction(proto) => self.compile_fun(proto, Vec::new()),
    //         Stmt::Function(inner) => self.compile_fun(inner.proto.0, inner.body.0),
    //         Stmt::Expression(expr, _) => self.compile_expr(expr),
    //         Stmt::Return(expr, _) => self.compile_ret(expr),
    //         Stmt::VariableDecl { name, ty, value, .. } => self.compile_var_decl(name, value, ty),
    //         Stmt::VariableAssign { name, value, .. } => self.compile_assign(name.0, value),
    //         Stmt::Block(statements) => self.compile_block(statements),
    //         Stmt::While(cond, body) => self.compile_while(cond.0, body),
    //         Stmt::If(inner) => self.compile_if(inner),
    //     }
    // }

    // // Must add new instruction in every case
    // fn compile_expr(&mut self, expr: Expr) {
    //     match expr {
    //         Expr::Literal(value) => self.compile_constant(value),
    //         Expr::Unary { op, right, .. } => self.compile_unary(op, *right),
    //         Expr::Binary {
    //             left, op, right, ..
    //         } => self.compile_bin(*left, op, *right),
    //         Expr::Call { callee, args, .. } => self.compile_call(*callee, args),
    //         Expr::Variable(_, name, _) => self.compile_var(name),
    //     }
    // }

    // fn convert_ty(&self, old_ty: ty::Ty) -> Ty {
    //     match old_ty {
    //         ty::Ty::String => Ty::String,
    //         ty::Ty::Bool => Ty::Bool,
    //         ty::Ty::I32 => Ty::I32,
    //         ty::Ty::F64 => Ty::F64,
    //         ty::Ty::Void => Ty::Void,
    //         ty::Ty::Fun(_, _) => todo!(),
    //         ty::Ty::DeferTyCheck(_, _) => unreachable!(),
    //     }
    // }

    // fn compile_if(&mut self, mut r#if: If<Expr, Stmt>) {
    //     let body_len = r#if.then.body.len();
    //     if body_len > u32::MAX as usize {
    //         panic!("Then body too long")
    //     }

    //     let else_len = if r#if.else_ifs.len() == 0 {
    //         r#if.otherwise.len()
    //     } else {
    //         1
    //     };
    //     if else_len > u32::MAX as usize {
    //         panic!("Else body too long")
    //     }

    //     self.add_inst(Inst::Branch(body_len as u32, else_len as u32));
    //     self.compile_expr(r#if.then.condition.0);

    //     self.compile_statements(r#if.then.body);

    //     if r#if.else_ifs.len() != 0 {
    //         let r#if = If {
    //             then: Box::new(r#if.else_ifs.remove(0)),
    //             ..r#if
    //         };
    //         self.compile_if(r#if);
    //     } else {
    //         self.compile_statements(r#if.otherwise);
    //     }
    // }

    // fn compile_while(&mut self, cond: Expr, body: Vec<Spanned<Stmt>>) {
    //     if body.len() + 1 > u32::MAX as usize {
    //         panic!("Loop body too long")
    //     }
    //     self.add_inst(Inst::Loop {
    //         len: 1,
    //     });

    //     self.add_inst(Inst::Branch((body.len()+1) as u32, 1));
    //     self.compile_expr(cond);

    //     self.compile_statements(body);
    //     self.add_inst(Inst::Repeat);

    //     self.add_inst(Inst::Break);
    // }

    // fn compile_block(&mut self, statements: Vec<Spanned<Stmt>>) {
    //     let body_len = statements.len();
    //     if body_len > u32::MAX as usize {
    //         panic!("Block body too long")
    //     }

    //     self.add_inst(Inst::Block { len: body_len as u32 });
    //     self.compile_statements(statements);
    // }

    // fn compile_assign(&mut self, name: String, value: Expr) {
    //     self.add_inst(Inst::Assign);
    //     self.add_string(name);
    //     self.compile_expr(value);
    // }

    // fn compile_var_decl(&mut self, name: String, value: Expr, var_ty: ty::Ty) {
    //     let var_ty = self.convert_ty(var_ty);
    //     self.add_inst(Inst::VarDecl(var_ty));
    //     self.compile_expr(value);
    //     self.add_string(name);
    // }

    // fn compile_var(&mut self, name: String) {
    //     self.add_inst(Inst::Var);
    //     self.add_string(name);
    // }

    // fn compile_call(&mut self, callee: Expr, args: Vec<Expr>) {
    //    self.add_inst(Inst::Call { arg_len: args.len() as u8 });
    //    self.compile_expr(callee);
    //    for arg in args {
    //     self.compile_expr(arg);
    //    }
    // }

    // fn compile_ret(&mut self, expr: Option<Expr>) {
    //     self.add_inst(Inst::Ret);

    //     match expr {
    //         Some(expr) => self.compile_expr(expr),
    //         None => self.add_inst(Inst::None) 
    //     }
    // }

    // fn compile_bin(&mut self, left: Expr, op: BinaryOp, right: Expr) {
    //     let inst = match op {
    //         BinaryOp::Sum => Inst::Sum,
    //         BinaryOp::Sub => Inst::Sub,
    //         BinaryOp::Mul => Inst::Mul,
    //         BinaryOp::Div => Inst::Div,
    //         BinaryOp::Rem => Inst::Rem,
    //         BinaryOp::Equal => Inst::Eq,
    //         BinaryOp::NotEqual => Inst::Neq,
    //         BinaryOp::Gt => Inst::Gt,
    //         BinaryOp::Lt => Inst::Lt,
    //         BinaryOp::Gte => Inst::Gte,
    //         BinaryOp::Lte => Inst::Lte,
    //         BinaryOp::LogicAnd => Inst::LogicAnd,
    //         BinaryOp::LogicOr => Inst::LogicOr,
    //     };

    //     self.add_inst(inst);
    //     self.compile_expr(left);
    //     self.compile_expr(right);
    // }

    // fn compile_unary(&mut self, op: UnaryOp, right: Expr) {
    //     let inst = match op {
    //         UnaryOp::Neg => Inst::Neg,
    //         UnaryOp::Not => Inst::Not,
    //     };

    //     self.add_inst(inst);
    //     self.compile_expr(right);
    // }

    // fn compile_constant(&mut self, value: Value) {
    //     let inst = match value {
    //         Value::String(v) => {
    //             self.add_string(v);
    //             Inst::String
    //         }
    //         Value::I32(v) => Inst::I32(v),
    //         Value::F64(v) => Inst::F64(v),
    //         Value::Bool(v) => Inst::Bool(v),
    //     };

    //     self.add_inst(inst);
    // }

    // fn compile_fun(&mut self, proto: ProtoFunction, body: Vec<Spanned<Stmt>>) {
    //     let params_len = proto.params.len();
    //     let body_len = body.len();
    //     if body_len > u32::MAX as usize {
    //         panic!("Body too long")
    //     }

    //     self.add_inst(Inst::Fun {
    //         params_len: params_len as u8,
    //         body_len: body_len as u32,
    //     });

    //     self.add_string(proto.name);
    //     let return_ty = self.convert_ty(proto.ty.fun_return_ty());
    //     self.add_data(Extra::Type(return_ty));
        
    //     for (_, name, ty) in proto.params {
    //         self.compile_field(name, ty);
    //     }

    //     self.compile_statements(body);
    // }

    // fn compile_field(&mut self, name: String, field_ty: ty::Ty) {
    //     let field_ty = self.convert_ty(field_ty);
    //     self.add_data(Extra::Type(field_ty));
    //     self.add_string(name);
    // }

    // fn add_inst(&mut self, inst: Inst) {
    //     self.header.instructions.push(inst)
    // }

    // fn add_string(&mut self, s: String) {
    //     let s = CString::new(s).expect("Proper C string");
    //     let mut bytes = s.as_c_str().to_bytes_with_nul().to_vec();

    //     self.header.strings.append(&mut bytes);
    // }

    // fn add_data(&mut self, data: Extra) {
    //     self.header.extra.push(data);
    // }
}
