use app::App;
use eframe::NativeOptions;

mod app;

fn main() {
    let options = NativeOptions::default();

    eframe::run_native("Mathool", options, Box::new(|cc| Box::new(App::new(cc))));
}
