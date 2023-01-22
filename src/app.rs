use std::{
    fs::File,
    io::{BufReader, Read},
};

use eframe::{
    egui::{CentralPanel, Context, FontData, FontDefinitions},
    epaint::{FontFamily, FontId},
    CreationContext, Frame,
};

use crate::style::{CustomFont, CustomStyle};

#[derive(Default)]
pub struct App {}

impl App {
    pub fn new(cc: &CreationContext) -> Self {
        let app = Self::default();
        app.set_fonts(&cc.egui_ctx);
        app
    }

    fn set_fonts(&self, ctx: &Context) {
        let mut fonts = FontDefinitions::default();
        let mut style = (*ctx.style()).clone();

        let paths = std::fs::read_dir("./fonts").unwrap();

        for path in paths {
            let entry = path.unwrap();
            let f = File::open(entry.path()).unwrap();
            let mut reader = BufReader::new(f);
            let mut buffer = Vec::new();

            // Read file into vector.
            reader.read_to_end(&mut buffer).unwrap();

            let os_fontname = entry.file_name();
            let fontname = os_fontname.to_str().unwrap();

            fonts.font_data.insert(
                fontname[0..(fontname.len() - 4)].into(),
                FontData::from_owned(buffer),
            );
        }

        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, "cmunrm".into());

        fonts
            .families
            .entry(CustomFont::TextArea.into())
            .or_default()
            .insert(0, "cmunrm".into());

        style.text_styles.insert(
            CustomStyle::TextArea.into(),
            FontId::new(32., CustomFont::TextArea.into()),
        );

        ctx.set_fonts(fonts);
        ctx.set_style(style)
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World!");
        });
    }
}
