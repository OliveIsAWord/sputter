#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Int(i64),
    Identifier(String),
    Expr(Vec<Value>),
}
