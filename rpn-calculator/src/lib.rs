#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for input in inputs {
        use CalculatorInput::*;
        if let Some(res) = match *input {
            Value(val) => {
                stack.push(val);
                None
            }
            _ => {
                if let (Some(v1), Some(v2)) = (stack.pop(), stack.pop()) {
                    let res = match *input {
                        Add => v1 + v2,
                        Subtract => v2 - v1,
                        Multiply => v2 * v1,
                        _ => v2 / v1,
                    };
                    Some(res)
                } else {
                    break;
                }
            }
        } {
            stack.push(res)
        };
    }
    if stack.len() != 1 {
        return None;
    }
    return stack.pop();
}
