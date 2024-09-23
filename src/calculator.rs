use crate::inputs::Inputs;
use crate::inputs::Inputs::*;
use eframe::{
    egui::{self},
    App,
};
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

    fn update_display(&mut self, input: &Inputs) {
        match input {
            Number(x) => self
                .display_value
                .push(char::from_digit(*x as u32, 10).unwrap()),
            Add => self.display_value.push('+'),
            Subtract => self.display_value.push('-'),
            Multiply => self.display_value.push('*'),
        }
    }

    fn get_postfix(&mut self) -> Vec<Inputs> {
        let mut postfix_vec = vec![];
        let mut operator_stack = vec![];
        while self.inputs.len() > 0 {
            match self.inputs.pop_front().unwrap() {
                Number(x) => postfix_vec.push(Number(x)),
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
                        .push_back(Number(new_digit + existing_digits * 10));
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
            _ => match self.inputs.pop_back() {
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
            },
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
                Add => output_stack.pop().unwrap() + output_stack.pop().unwrap(),
                Subtract => output_stack.pop().unwrap() - output_stack.pop().unwrap(),
                Multiply => output_stack.pop().unwrap() * output_stack.pop().unwrap(),
            };
            output_stack.push(next_value)
        }
        let result = output_stack.pop().unwrap();
        self.inputs.push_back(Number(result));
        self.display_value = result.to_string();
    }

    pub fn clear(&mut self) {
        self.inputs = VecDeque::new();
        self.display_value = String::from("");
    }
}

// The UI for the calculator
impl App for Calculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(&self.display_value);
            egui::Grid::new("grid").show(ui, |ui| {
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("7"))
                    .clicked()
                {
                    self.add_input(Inputs::Number(7))
                }
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("8"))
                    .clicked()
                {
                    self.add_input(Inputs::Number(8))
                }
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("9"))
                    .clicked()
                {
                    self.add_input(Inputs::Number(9))
                }
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("*"))
                    .clicked()
                {
                    self.add_input(Inputs::Multiply)
                }
                ui.end_row();
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("4"))
                    .clicked()
                {
                    self.add_input(Inputs::Number(4))
                }
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("5"))
                    .clicked()
                {
                    self.add_input(Inputs::Number(5))
                }
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("6"))
                    .clicked()
                {
                    self.add_input(Inputs::Number(6))
                }
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("-"))
                    .clicked()
                {
                    self.add_input(Inputs::Subtract)
                }
                ui.end_row();
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("1"))
                    .clicked()
                {
                    self.add_input(Inputs::Number(1))
                }
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("2"))
                    .clicked()
                {
                    self.add_input(Inputs::Number(2))
                }
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("3"))
                    .clicked()
                {
                    self.add_input(Inputs::Number(3))
                }
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("+"))
                    .clicked()
                {
                    self.add_input(Inputs::Add)
                }
                ui.end_row();
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("clr"))
                    .clicked()
                {
                    self.clear();
                }
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("0"))
                    .clicked()
                {
                    self.add_input(Inputs::Number(0))
                }
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("="))
                    .clicked()
                {
                    self.evaluate();
                };
            });

            // dbg!(&self.inputs);
        });
    }
}
