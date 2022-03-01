use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Int(BigInt),
    Identifier(String),
    Expr(Vec<Value>),
}
