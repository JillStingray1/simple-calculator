mod calculator;
mod inputs;

use calculator::Calculator;
use eframe::{
    egui::{self},
    App,
};
use eframe::{run_native, NativeOptions};

fn main() {
    let app = Calculator::new();
    let win_option = NativeOptions::default();
    run_native("Calculator", win_option, Box::new(|_cc| Ok(Box::new(app))))
        .unwrap();
}

// The UI for the calculator
impl App for Calculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        use inputs::Inputs::*;
        ctx.set_pixels_per_point(1.5);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(&self.display_value);
            egui::Grid::new("grid").show(ui, |ui| {
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("7"))
                    .clicked()
                {
                    self.add_input(Number(7.0))
                }
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("8"))
                    .clicked()
                {
                    self.add_input(Number(8.0))
                }
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("9"))
                    .clicked()
                {
                    self.add_input(Number(9.0))
                }
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("*"))
                    .clicked()
                {
                    self.add_input(Multiply)
                }
                ui.end_row();
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("4"))
                    .clicked()
                {
                    self.add_input(Number(4.0))
                }
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("5"))
                    .clicked()
                {
                    self.add_input(Number(5.0))
                }
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("6"))
                    .clicked()
                {
                    self.add_input(Number(6.0))
                }
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("-"))
                    .clicked()
                {
                    self.add_input(Subtract)
                }
                ui.end_row();
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("1"))
                    .clicked()
                {
                    self.add_input(Number(1.0))
                }
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("2"))
                    .clicked()
                {
                    self.add_input(Number(2.0))
                }
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("3"))
                    .clicked()
                {
                    self.add_input(Number(3.0))
                }
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("+"))
                    .clicked()
                {
                    self.add_input(Add)
                }
                ui.end_row();
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("clr"))
                    .clicked()
                {
                    self.clear();
                }
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("."))
                    .clicked()
                {
                    self.add_input(Decimal);
                }
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("0"))
                    .clicked()
                {
                    self.add_input(Number(0.0))
                }
                if ui
                    .add_sized([100.0, 50.0], egui::Button::new("/"))
                    .clicked()
                {
                    self.add_input(Divide);
                };
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
