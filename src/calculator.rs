use crate::inputs::Inputs;
use crate::inputs::Inputs::*;

use std::collections::VecDeque;

// Logic for the evaluation of the calculator.
pub struct Calculator {
    pub display_value: String,
    pub inputs: VecDeque<Inputs>,
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator {
            display_value: String::from(""),
            inputs: VecDeque::new(),
        }
    }

    pub fn clear(&mut self) {
        self.inputs.clear();
        self.display_value.clear();
    }

    fn update_display(&mut self, input: &Inputs) {
        match input {
            Number(x) => self
                .display_value
                .push(char::from_digit(*x as u32, 10).unwrap()),
            Decimal => self.display_value.push('.'),
            Add => self.display_value.push('+'),
            Subtract => self.display_value.push('-'),
            Multiply => self.display_value.push('*'),
            Divide => self.display_value.push('/'),
            LeftParen => self.display_value.push('('),
            RightParen => self.display_value.push(')'),
        }
    }

    fn get_postfix(&mut self) -> Vec<Inputs> {
        let mut postfix_vec = vec![];
        let mut operator_stack = vec![];
        while self.inputs.len() > 0 {
            match self.inputs.pop_front().unwrap() {
                Number(x) => postfix_vec.push(Number(x)),
                LeftParen => todo!(),
                RightParen => todo!(),
                input => match operator_stack.pop() {
                    Some(previous) => {
                        if previous >= input {
                            postfix_vec.push(previous);
                        } else {
                            operator_stack.push(previous);
                        }
                        operator_stack.push(input)
                    }
                    None => operator_stack.push(input),
                },
            }
        }
        while operator_stack.len() > 0 {
            postfix_vec.push(operator_stack.pop().unwrap());
        }
        return postfix_vec;
    }

    pub fn add_input(&mut self, input: Inputs) {
        match input {
            Number(new_digit) => match self.inputs.pop_back() {
                Some(Number(existing_digits)) => {
                    self.inputs
                        .push_back(Number(new_digit + existing_digits * 10.0));
                    self.update_display(&input);
                }
                Some(previous) => {
                    self.update_display(&input);
                    self.inputs.push_back(previous);
                    self.inputs.push_back(input);
                }
                None => {
                    self.update_display(&input);
                    self.inputs.push_back(input);
                }
            },
            _ => {
                match self.inputs.pop_back() {
                    Some(Number(previous)) => {
                        self.update_display(&input);
                        self.inputs.push_back(Number(previous));
                        self.inputs.push_back(input)
                    }
                    Some(x) => {
                        self.inputs.push_back(x);
                        eprintln!("2 adjacent operators, try inputing a number between")
                    }
                    None => eprintln!("Operator inputed with no numbers"),
                }
            }
        }
    }

    pub fn evaluate(&mut self) {
        match self.inputs.back() {
            Some(Number(_)) => (),
            _ => {
                eprintln!("Incomplete expression.");
                return;
            }
        }
        let postfix_vec = self.get_postfix();
        let mut output_stack = vec![];
        for value in postfix_vec.iter() {
            let next_value = match value {
                Number(x) => *x,
                operator => {
                    let right = output_stack.pop().unwrap();
                    let left = output_stack.pop().unwrap();
                    match operator {
                        Add => left + right,
                        Subtract => left - right,
                        Multiply => left * right,
                        Divide => left / right,
                        Decimal => {
                            let mut digits = 1;
                            let mut temp = right;
                            while temp > 10.0 {
                                temp = temp / 10.0;
                                digits += 1;
                            }
                            left + right * f64::powi(10.0, -digits)
                        }
                        _ => 0.0,
                    }
                }
            };
            output_stack.push(next_value)
        }
        let result = output_stack.pop().unwrap();
        self.inputs.push_back(Number(result));
        self.display_value = result.to_string();
    }
}
