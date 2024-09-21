use crate::inputs::Inputs;
use eframe::{
    egui::{self},
    App,
};

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
                Some(Number(existing_digits)) => {
                    self.inputs.push(Number(new_digit + existing_digits * 10));
                    self.update_display(&input);
                }
                Some(previous) => {
                    self.update_display(&input);
                    self.inputs.push(previous);
                    self.inputs.push(input);
                }
                None => {
                    self.update_display(&input);
                    self.inputs.push(input);
                }
            },
            _ => match self.inputs.pop() {
                Some(Number(previous)) => {
                    self.update_display(&input);
                    self.inputs.push(Number(previous));
                    self.inputs.push(input)
                }
                Some(x) => {
                    self.inputs.push(x);
                    eprintln!("2 adjacent operators, try inputing a number between")
                }
                None => eprintln!("Operator inputed with no numbers"),
            },
        }
    }

    pub fn evaluate(&mut self) {}
}

impl App for Calculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let ui = egui::CentralPanel::default();
        ui.show(ctx, |ui| {
            ui.label(&self.display_value);
            ui.horizontal(|ui| {
                if ui.button("7").clicked() {
                    self.add_input(Inputs::Number(7))
                }
                if ui.button("8").clicked() {
                    self.add_input(Inputs::Number(8))
                }
                if ui.button("9").clicked() {
                    self.add_input(Inputs::Number(9))
                }
                if ui.button("*").clicked() {
                    self.add_input(Inputs::Multiply)
                }
            });
            ui.horizontal(|ui| {
                if ui.button("4").clicked() {
                    self.add_input(Inputs::Number(4))
                }
                if ui.button("5").clicked() {
                    self.add_input(Inputs::Number(5))
                }
                if ui.button("6").clicked() {
                    self.add_input(Inputs::Number(6))
                }
                if ui.button("-").clicked() {
                    self.add_input(Inputs::Subtract)
                }
            });
            ui.horizontal(|ui| {
                if ui.button("1").clicked() {
                    self.add_input(Inputs::Number(1))
                }
                if ui.button("2").clicked() {
                    self.add_input(Inputs::Number(2))
                }
                if ui.button("3").clicked() {
                    self.add_input(Inputs::Number(3))
                }
                if ui.button("+").clicked() {
                    self.add_input(Inputs::Add)
                }
            });
            ui.horizontal(|ui| {
                if ui.button("0").clicked() {
                    self.add_input(Inputs::Number(0))
                }
                if ui.button("=").clicked() {
                    self.add_input(Inputs::Number(2))
                }
            });

            dbg!(&self.inputs);
        });
    }
}
