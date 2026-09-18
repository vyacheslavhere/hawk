/// Imports
use crate::common::span::Span;

/// Defines an unary operation
#[derive(Debug, Clone, PartialEq)]
pub enum UnOp {
    Neg, // -
    Not, // !
}

/// Defines a binary operation
#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,    // +
    Sub,    // -
    Mul,    // *
    Div,    // /
    Rem,    // %
    Xor,    // ^
    Gt,     // >
    Ge,     // >=
    Lt,     // <
    Le,     // <=
    Eq,     // ==
    Ne,     // !=
    BitOr,  // |
    BitAnd, // &
    Or,     // ||
    And,    // &&
    Shl,    // <<
    Shr,    // >>
}

/// Defines a literal
#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Number(String),
    String(String),
    True,
    False,
    Nil,
}

/// Defines an identifier
#[derive(Debug, Clone, PartialEq)]
pub enum Ident {
    Wildcard,
    Id(String),
}

/// Defines a rest part
#[derive(Debug, Clone, PartialEq)]
pub enum Rest {
    Ignore,
    Bind(String),
}

/// Defines an array element
#[derive(Debug, Clone, PartialEq)]
pub enum ArrayElement {
    Value(Box<Expr>),
    Spread(Box<Expr>),
}

/// Defines a dictionary element
#[derive(Debug, Clone, PartialEq)]
pub enum DictElement {
    Pair(Box<Expr>, Box<Expr>),
    Spread(Box<Expr>),
}

/// Defines a match pattern kind
#[derive(Debug, Clone, PartialEq)]
pub enum PatKind {
    Lit(Lit),
    Array {
        items: Vec<Ident>,
        rest: Option<Rest>,
    },
    Dict {
        pairs: Vec<(Ident, Ident)>,
        rest: Option<Rest>,
    },
    Range {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        inclusive: bool,
    },
    Bind(String),
}

/// Defines a match pattern
#[derive(Debug, Clone, PartialEq)]
pub struct Pat {
    pub span: Span,
    pub kind: PatKind,
}

/// Defines a match case
#[derive(Debug, Clone, PartialEq)]
pub struct Case {
    pub span: Span,
    pub pat: Pat,
    pub then: Box<Expr>,
}

/// Defines an expression kind
#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Lit(Lit),
    Array(Vec<ArrayElement>),
    Dict(Vec<DictElement>),
    Range {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        inclusive: bool,
    },
    Id(Ident),
    Field(Box<Expr>, Ident),
    Index(Box<Expr>, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    Un(UnOp, Box<Expr>),
    Bin(BinOp, Box<Expr>, Box<Expr>),
    If {
        condition: Box<Expr>,
        then: Box<Expr>,
        else_: Option<Box<Expr>>,
    },
    Memoize(Box<Expr>),
    Match {
        scrutinee: Box<Expr>,
        cases: Vec<Case>,
    },
    Block(Vec<Stmt>),
    Function {
        params: Vec<String>,
        body: Box<Expr>,
    },
    Use(String),
    Pipe(Box<Expr>, Box<Expr>),
}

/// Defines an expression
#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub span: Span,
    pub kind: ExprKind,
}

/// Defines a statement kind
#[derive(Debug, Clone, PartialEq)]
pub enum StmtKind {
    Bind(Ident, Box<Expr>),
    Echo(Box<Expr>),
    Expr(Box<Expr>),
}

/// Defines a statement
#[derive(Debug, Clone, PartialEq)]
pub struct Stmt {
    pub span: Span,
    pub kind: StmtKind,
}

/// Defines a program
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}
