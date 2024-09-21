#[derive(Debug)]
pub enum Inputs {
    Number(i64),
    Add,
    Subtract,
    Multiply,
}

pub struct Calculator {
    pub display_value: String,
    pub inputs: Vec<Inputs>,
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator {
            display_value: String::from(""),
            inputs: vec![],
        }
    }

    fn update_display(&mut self, input: &Inputs) {
        use Inputs::*;
        match input {
            Number(x) => self
                .display_value
                .push(char::from_digit(*x as u32, 10).unwrap()),
            Add => self.display_value.push('+'),
            Subtract => self.display_value.push('-'),
            Multiply => self.display_value.push('-'),
        }
    }

    fn shunting_yard(&mut self, input: Inputs) {}

    pub fn add_input(&mut self, input: Inputs) {
        use Inputs::*;
        match input {
            Number(new_digit) => match self.inputs.pop() {
                Some(previous) => match &previous {
                    Number(existing_digits) => {
                        self.inputs.push(Number(new_digit + existing_digits * 10));
                        self.update_display(&input);
                    }
                    _ => {
                        self.update_display(&input);
                        self.inputs.push(previous);
                        self.inputs.push(input);
                    }
                },
                None => {
                    self.update_display(&input);
                    self.inputs.push(input);
                }
            },
            _ => match self.inputs.pop() {
                Some(previous) => match &previous {
                    Number(_) => {
                        self.update_display(&input);
                        self.inputs.push(previous);
                        self.inputs.push(input)
                    }
                    _ => {
                        self.inputs.push(previous);
                        eprintln!("2 adjacent operators, try inputing a number between")
                    }
                },
                None => eprintln!("Operator inputed with no numbers"),
            },
        }
    }

    pub fn evaluate(&mut self) {}
}

// fn do_operation(operation: Inputs, num_1: i64, num_2: i64) -> i64 {}
