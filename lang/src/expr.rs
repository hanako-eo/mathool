#[derive(Debug)]
pub enum Expr {
    FunctionCall(String, Vec<Expr>),
    FunctionDefinition(String, Vec<String>),
    Symbol(String),
    Number(i64),

    // Operations
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),

    // Sugar math syntax
    Abs(Box<Expr>),
}
