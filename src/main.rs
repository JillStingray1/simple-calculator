mod calculator;

use calculator::{Calculator, Inputs};
use eframe::{
    egui::{self},
    run_native, App, NativeOptions,
};

fn main() {
    let app = Calculator::new();
    let win_option = NativeOptions::default();
    run_native("Calculator", win_option, Box::new(|_cc| Ok(Box::new(app)))).unwrap();
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
