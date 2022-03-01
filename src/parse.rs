use std::iter::Peekable;
use std::str::Chars;

use crate::value::Value;

// ()
// (some)
// (lots of values with no limits)
// (and (you can) recurse (as (much (as (you (want))))))
// (((any) part) of the expression ((too)))
// (a) = a

const OPEN_PAREN: char = '(';
const CLOSE_PAREN: char = ')';

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ParseError {
    ExpectedOpenParen,
    UnclosedParen,
    ExpectedEOF,
}
use ParseError::*;

pub type Result<T> = std::result::Result<T, ParseError>;

// Returns the next non-whitespace character,
// as well as whether the current token should be terminated.
fn next_skip_whitespace(s: &mut Peekable<Chars<'_>>) -> (Option<char>, bool) {
    let mut has_whitespace = false;
    while s.next_if(|c| c.is_whitespace()).is_some() {
        has_whitespace = true;
    }
    let c = s.next();
    let is_delimiter = has_whitespace || c == Some(OPEN_PAREN) || c == Some(CLOSE_PAREN);
    (c, is_delimiter)
}

fn parse_internal(s: &mut Peekable<Chars<'_>>) -> Result<Value> {
    let mut children = vec![];
    let mut token = String::new();
    while let (Some(c), is_delimiter) = next_skip_whitespace(s) {
        if is_delimiter && !token.is_empty() {
            let child = string_to_value(token);
            children.push(child);
            token = String::new();
        }
        if c == OPEN_PAREN {
            children.push(parse_internal(s)?);
        } else if c == CLOSE_PAREN {
            return Ok(Value::Expr(children));
        } else {
            token.push(c)
        }
    }
    Err(UnclosedParen)
}

pub fn parse(s: &str) -> Result<Value> {
    let mut s = s.chars().peekable();
    let first_char = next_skip_whitespace(&mut s).0;
    if first_char == Some(OPEN_PAREN) {
        let expr = parse_internal(&mut s)?;
        if s.peek() == None || next_skip_whitespace(&mut s).0 == None {
            Ok(expr)
        } else {
            Err(ExpectedEOF)
        }
    } else if first_char == None {
        Ok(Value::Expr(vec![]))
    } else {
        Err(ExpectedOpenParen)
    }
}

fn string_to_value(x: String) -> Value {
    if let Ok(num) = x.parse() {
        Value::Int(num)
    } else {
        let mut ident = x;
        ident.shrink_to_fit();
        Value::Identifier(ident)
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use Tree::*;

    #[test]
    fn empty_string() {
        assert_eq!(parse(""), Err(ExpectedOpenParen));
    }
    #[test]
    fn empty_expr() {
        assert_eq!(parse("()").unwrap(), Tree::Children(vec![]));
    }
    #[test]
    fn abc() {
        let a = Tree::End('a'.into());
        let b = Tree::End('b'.into());
        let c = Tree::End('c'.into());
        assert_eq!(parse("(a b c)").unwrap(), Tree::Children(vec![a, b, c]));
    }
    #[test]
    fn whitespace() {
        let a = Tree::End('a'.into());
        let b = Tree::End('b'.into());
        let c = Tree::End('c'.into());
        let d = Tree::End('d'.into());
        let e = Tree::End('e'.into());
        assert_eq!(
            parse("   (    a\nb\tc       d\n\t  \n\n\r\n\te)").unwrap(),
            Tree::Children(vec![a, b, c, d, e])
        );
    }
    #[test]
    fn recursion_basic() {
        let a = Tree::End('a'.into());
        let b = Tree::End('b'.into());
        let c = Tree::End('c'.into());
        assert_eq!(
            parse("((a b) c)").unwrap(),
            Tree::Children(vec![Tree::Children(vec![a, b]), c])
        );
    }
    #[test]
    fn unicode_basic() {
        let a = Tree::End("ένας".into());
        let b = Tree::End("สอง".into());
        let c = Tree::End("trì".into());
        let d = Tree::End("𝄞".into());
        assert_eq!(
            parse("((ένας สอง) trì 𝄞)").unwrap(),
            Tree::Children(vec![Tree::Children(vec![a, b]), c, d])
        );
    }
    #[test]
    fn recursion_levels() {
        let a = Tree::End('a'.into());
        assert_eq!(
            parse("(((a)))").unwrap(),
            Children(vec![Children(vec![Children(vec![a])])])
        );
    }
    #[test]
    fn factorial() {
        assert_eq!(
            parse("(defun factorial (x) (if (zerop x) 1 (* x (factorial (- x 1)))))").unwrap(),
            Children(vec![
                End("defun".into()),
                End("factorial".into()),
                Children(vec![End("x".into())]),
                Children(vec![
                    End("if".into()),
                    Children(vec![End("zerop".into()), End("x".into())]),
                    End("1".into()),
                    Children(vec![
                        End("*".into()),
                        End("x".into()),
                        Children(vec![
                            End("factorial".into()),
                            Children(vec![End("-".into()), End("x".into()), End("1".into())])
                        ])
                    ])
                ])
            ])
        );
    }
    #[test]
    fn fibonacci() {
        // I hope you can forgive the use of Google Translate
        assert_eq!(
            parse(
                "(정의하다 斐波那契 (Икс) (もしも (< Икс 2) 1 (+ (斐波那契 (- Икс 1)) (斐波那契 (- Икс 2)))))"
            ).unwrap(),
            Children(vec![
                End("정의하다".into()),
                End("斐波那契".into()),
                Children(vec![End("Икс".into())]),
                Children(vec![
                    End("もしも".into()),
                    Children(vec![End("<".into()), End("Икс".into()), End("2".into())]),
                    End("1".into()),
                    Children(vec![
                        End("+".into()),
                        Children(vec![
                            End("斐波那契".into()),
                            Children(vec![End("-".into()), End("Икс".into()), End("1".into())])
                        ]),
                        Children(vec![
                            End("斐波那契".into()),
                            Children(vec![End("-".into()), End("Икс".into()), End("2".into())])
                        ])
                    ])
                ])
            ])
        );
    }
}
*/
