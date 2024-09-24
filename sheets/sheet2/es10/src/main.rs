use std::fmt::*;
// use std::result::*;

#[allow(dead_code)]
enum Operation {
    Add, 
    Sub,
    Mul,
    Div,
}

enum Expression {
    Number(i32),
    Operation(Box<Expression>, Box<Expression>, Operation),
}

impl Display for Operation {
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
match self {
Operation::Add => write!(f, "+"),
Operation::Sub => write!(f, "-"),
Operation::Mul => write!(f, "*"),
Operation::Div => write!(f, "/"),
}
}
}

// just to have a nice output, not required for the exercise
impl Display for Expression {
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
match self {
Expression::Operation ( left, op, right ) => {
write!(f, "({} {} {})", left, op, right)
}
Expression::Number(n) => write!(f, "{}", n),
}
}
}


fn evaluate(exp: &Expression) -> std::result::Result<i32, String> {
    match exp {
        Expression::Number(n) => Ok(*n),
        Expression::Operation(left, right, Operation::Add) 
            => {
                match evaluate(left)?.checked_add(evaluate(right)?) {
                    Some(v) => Ok(v),
                    None => Err("overflow".to_string()),
                }
            }
        Expression::Operation(left, right, Operation::Sub) 
            => {
                match evaluate(left)?.checked_sub(evaluate(right)?) {
                    Some(v) => Ok(v),
                    None => Err("overflow".to_string()),
                }
            }
        Expression::Operation(left, right, Operation::Mul) 
            => {
                match evaluate(left)?.checked_mul(evaluate(right)?) {
                    Some(v) => Ok(v),
                    None => Err("overflow".to_string()),
                }
            }
        Expression::Operation(left, right, Operation::Div) 
            => { 
                match evaluate(left)?.checked_div(evaluate(right)?) {
                    Some(v) => Ok(v),
                    None => Err("Division by 0".to_string()),
                }
            }
            
    }
}

fn main() {
    let e2 = &Expression::Operation(Box::new(Expression::Operation(Box::new(Expression::Number(2)), Box::new(Expression::Number(2)), Operation::Div)), Box::new(Expression::Number(2)), Operation::Div);
    println!("{:?}", evaluate(&e2));
}
