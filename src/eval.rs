use crate::value::Value;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum EvalError {
    BadArgument(usize),
}
use EvalError::*;

pub type Result<T> = std::result::Result<T, EvalError>;

pub fn eval(v: Value) -> Value {
    //println!("Evaluating {:?}", v);
    use Value::*;
    // If the value is not an expression, there is nothing to evaluate.
    //let Expr(expr) = v else { return v; };
    let expr = if let Expr(e) = v { e } else { return v };
    let evaled_terms: Vec<_> = expr.iter().cloned().map(eval).collect();
    let op = match evaled_terms.get(0) {
        Some(first) => {
            // an expression with 1 element evaluates to that element
            if evaled_terms.len() == 1 {
                return first.clone();
            } else if let Identifier(op) = first {
                op
            } else {
                return Expr(evaled_terms);
            }
        }
        _ => {
            return Expr(evaled_terms);
        }
    };
    //println!("Got {:?} -> {:?}", op, evaled_terms);
    let args = &evaled_terms[1..];
    assert!(!args.is_empty());
    match op.as_ref() {
        "+" => num_oper(|x, y| x + y, args).unwrap(),
        "-" => num_oper(|x, y| x - y, args).unwrap(),
        "*" => num_oper(|x, y| x * y, args).unwrap(),
        "/" => num_oper(|x, y| x / y, args).unwrap(),
        _ => return Expr(evaled_terms),
    }
}
use num_bigint::BigInt;
pub fn num_oper<F: Fn(BigInt, BigInt) -> BigInt>(func: F, args: &[Value]) -> Result<Value> {
    assert!(!args.is_empty());
    use Value::*;
    let mut cumulative = if let Int(num) = args[0].clone() {
        num
    } else {
        return Err(BadArgument(0));
    };
    for (i, arg) in args[1..].iter().enumerate() {
        if let Int(num) = arg {
            cumulative = func(cumulative, num.clone());
        } else {
            return Err(BadArgument(i + 1));
        }
    }
    Ok(Int(cumulative))
}
