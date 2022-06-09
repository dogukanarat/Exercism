#[derive(Debug, Eq, PartialEq)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<CalculatorInput> = Vec::new();
    let mut result: Option<i32> = None;

    for input in inputs {
        match input {
            CalculatorInput::Add => {
                if stack.len() < 2 {
                    return None;
                }
                let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());

                let result_add = match a {
                    CalculatorInput::Value(a) => match b {
                        CalculatorInput::Value(b) => {
                            Some(CalculatorInput::Value(a + b))
                        }
                        _ => None,
                    },
                    _ => None,
                };

                if result_add.is_some() {
                    stack.push(result_add.unwrap());
                } else {
                    return None;
                }
            }
            CalculatorInput::Subtract => {
                if stack.len() < 2 {
                    return None;
                }
                let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());

                let result_subtract = match a {
                    CalculatorInput::Value(a) => match b {
                        CalculatorInput::Value(b) => {
                            Some(CalculatorInput::Value(b - a))
                        }
                        _ => None,
                    },
                    _ => None,
                };

                if result_subtract.is_some() {
                    stack.push(result_subtract.unwrap());
                } else {
                    return None;
                }
            }
            CalculatorInput::Multiply => {
                if stack.len() < 2 {
                    return None;
                }
                let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());

                let result_multiply = match a {
                    CalculatorInput::Value(a) => match b {
                        CalculatorInput::Value(b) => {
                            Some(CalculatorInput::Value(a * b))
                        }
                        _ => None,
                    },
                    _ => None,
                };

                if result_multiply.is_some() {
                    stack.push(result_multiply.unwrap());
                } else {
                    return None;
                }
            }
            CalculatorInput::Divide => {
                if stack.len() < 2 {
                    return None;
                }
                let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());

                let result_divide = match a {
                    CalculatorInput::Value(a) => match b {
                        CalculatorInput::Value(b) => {
                            Some(CalculatorInput::Value(b / a))
                        }
                        _ => None,
                    },
                    _ => None,
                };

                if result_divide.is_some() {
                    stack.push(result_divide.unwrap());
                } else {
                    return None;
                }
            }
            CalculatorInput::Value(v) => {
                stack.push(CalculatorInput::Value(*v));
            }
        }
    }

    if stack.len() == 1 {
        if let Some(final_result) = stack.pop() {
            match final_result {
                CalculatorInput::Value(v) => result = Some(v),
                _ => result = None,
            }
        }
    }

    result
}
