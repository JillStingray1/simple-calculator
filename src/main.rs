mod calculator;
mod inputs;

use calculator::Calculator;
use eframe::{run_native, NativeOptions};

fn main() {
    let app = Calculator::new();
    let win_option = NativeOptions::default();
    run_native("Calculator", win_option, Box::new(|_cc| Ok(Box::new(app)))).unwrap();
}
