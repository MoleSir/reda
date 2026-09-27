use std::{path::Path, str::FromStr};
use crate::parse::{self, ParseError};

#[derive(Debug)]
pub struct Verilog {
    pub modules: Vec<Module>,
}

impl Verilog {
    pub fn load_file<P: AsRef<Path>>(path: P) -> Result<Self, ParseError> {
        parse::load_file(path)
    }
}

impl FromStr for Verilog {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse::load_str(s)
    }
}

/// 模块声明
#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub port_names: Vec<String>,
    pub ports: Vec<Port>,
    pub nets: Vec<Net>,
    pub parameters: Vec<Parameter>,
    pub assign: Vec<Assign>,
    pub always: Vec<Always>,
    pub instances: Vec<Instance>,
}

/// 模块端口
#[derive(Debug)]
pub struct Port {
    pub name: String,
    pub direction: PortDirection,
    pub width: Option<Range>, // e.g. [7:0]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortDirection {
    Input,
    Output,
    Inout,
}

/// 范围 [msb:lsb]
#[derive(Debug, PartialEq, Eq)]
pub struct Range {
    pub msb: i32,
    pub lsb: i32,
}

/// 信号声明
#[derive(Debug)]
pub struct Net {
    pub kind: NetKind, 
    pub name: String,
    pub width: Option<Range>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NetKind {
    Wire,
    Reg,
}

/// 参数声明
#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub value: ConstExpr,
}

/// assign 语句
#[derive(Debug)]
pub struct Assign {
    pub lhs: VarExpr,
    pub rhs: Expr,
}

/// always 块
#[derive(Debug)]
pub struct Always {
    pub sensitivity: SensitivityList,
    pub stmts: Vec<Stmt>,
}

/// always 的触发条件
#[derive(Debug)]
pub enum SensitivityList {
    Star,                          // always @(*)
    List(Vec<SensitivityEvent>),   // always @(posedge clk, negedge rst)
}

#[derive(Debug)]
pub struct SensitivityEvent {
    pub edge: Option<EdgeKind>,
    pub signal: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeKind {
    Posedge,
    Negedge,
}

/// 子模块实例化
#[derive(Debug)]
pub struct Instance {
    pub module_name: String,
    pub instance_name: String,
    pub connections: Vec<Connection>,
}

#[derive(Debug)]
pub struct Connection {
    pub port: String,
    pub expr: Expr,
}

#[derive(Debug)]
pub enum VarExpr {
    Ident(String),
    RangeSelect { base: String, range: Range },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Ident(String),
    Number(i32),
    Value(Value),
    RangeSelect { base: String, range: Range },
    BinOp { op: BinOpKind, lhs: Box<Expr>, rhs: Box<Expr> },
    UnaryOp { op: UnaryOpKind, expr: Box<Expr> },
}

#[derive(Debug)]
pub enum ConstExpr {
    Number(i32),
    Value(Value),
}

#[derive(Debug, PartialEq, Eq)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    And,
    Or,
    Eq,
    Neq,
    Lt,
    Gt,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnaryOpKind {
    Not,
    Neg,
}

/// 语句 (用于 always 块)
#[derive(Debug)]
pub enum Stmt {
    BlockingAssign { lhs: VarExpr, rhs: Expr },
    NonBlockingAssign { lhs: VarExpr, rhs: Expr },
    If { cond: Expr, then_body: Vec<Stmt>, else_body: Option<Vec<Stmt>> },
    Case { expr: Expr, items: Vec<(Expr, Vec<Stmt>)>, default: Option<Vec<Stmt>> },
}

#[derive(Debug, PartialEq, Eq)]
pub enum ValueBase {
    Bin,
    Oct,
    Dec,
    Hex,
}

pub enum ValueBit {
    X,
    Z,
    One,
    Zero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Value {
    pub len: usize,
    pub base: ValueBase,
    pub bits: String,
}