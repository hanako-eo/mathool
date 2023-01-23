use std::{
    fs::File,
    io::{BufReader, Read},
    ops::RangeFull,
};

use eframe::{
    egui::{
        plot::{Line, Plot, PlotPoints},
        CentralPanel, Context, FontData, FontDefinitions, Layout, SidePanel, TextEdit,
        TopBottomPanel,
    },
    emath::Align,
    epaint::{FontFamily, FontId, Vec2},
    CreationContext, Frame,
};

use crate::{
    style::{CustomFont, CustomStyle},
    widgets::selectable_list::SelectableList,
};

#[derive(Default, PartialEq)]
enum Tabs {
    #[default]
    Graphics,
    ComplexPlan,
}

#[derive(Default)]
pub struct App {
    tab: Tabs,
    formula: String,
}

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
            .entry(CustomFont::Text.into())
            .or_default()
            .insert(0, "cmunrm".into());

        style.text_styles.insert(
            CustomStyle::SelectableList.into(),
            FontId::new(16., CustomFont::Text.into()),
        );

        style.text_styles.insert(
            CustomStyle::TextArea.into(),
            FontId::new(24., CustomFont::Text.into()),
        );

        ctx.set_fonts(fonts);
        ctx.set_style(style)
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        TopBottomPanel::top("tabs").show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                SelectableList::new([
                    ("Graphics", Tabs::Graphics),
                    ("Complex Plan", Tabs::ComplexPlan),
                ])
                .font(CustomStyle::SelectableList)
                .show(ui, &mut self.tab);
            });
        });
        SidePanel::left("equations").show(ctx, |ui| {
            TextEdit::singleline(&mut self.formula)
                .font(CustomStyle::TextArea)
                .show(ui);
        });
        CentralPanel::default().show(ctx, |ui| {
            let Vec2 { x, y } = ui.available_size_before_wrap();
            let sin: PlotPoints =
                PlotPoints::from_explicit_callback(|x| x.sin(), RangeFull::default(), 10000);

            let line = Line::new(sin);
            Plot::new("functions")
                .width(x)
                .height(y)
                .allow_scroll(false)
                .show(ui, |plot_ui| plot_ui.line(line));
        });
    }
}
