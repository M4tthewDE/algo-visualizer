use algs::unionfind::{UnionFindAlg, UnionFindType};
use eframe::{App, Frame, NativeOptions};
use egui::Context;

mod algs;
mod ui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = NativeOptions::default();

    eframe::run_native(
        "Algo Visualizer",
        options,
        Box::new(|_cc| Box::<AlgoApp>::default()),
    )
}

#[derive(Default)]
pub struct AppData {
    n: u64,
    p_union: usize,
    q_union: usize,
    p_connected: usize,
    q_connected: usize,
    connected_text: Option<String>,
    union_find_type: UnionFindType,
    uf: Option<UnionFindAlg>,
    is_simulation: bool,
    iterations: u64,
    square_size: u64,
}

#[derive(Default)]
struct AlgoApp {
    app_data: AppData,
}

impl App for AlgoApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ui::show(ctx, &mut self.app_data);
    }
}
