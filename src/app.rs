use eframe::{
    egui::{CentralPanel, Context},
    CreationContext, Frame,
};

#[derive(Default)]
pub struct App {}

impl App {
    pub fn new(_cc: &CreationContext) -> Self {
        let app = Self::default();
        app
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World!");
        });
    }
}
