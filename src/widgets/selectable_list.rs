use eframe::{
    egui::{FontSelection, Response, RichText, SelectableLabel, Sense, Ui, Widget},
    epaint::Vec2,
};

pub struct SelectableList<E: PartialEq, T: Into<RichText>> {
    selectable: Vec<(T, E)>,

    font_selection: FontSelection,
}

impl<E: PartialEq, T: Into<RichText>> SelectableList<E, T> {
    pub fn new(selectable: impl Into<Vec<(T, E)>>) -> Self {
        Self {
            selectable: selectable.into(),
            font_selection: FontSelection::Default,
        }
    }

    pub fn font(mut self, font_selection: impl Into<FontSelection>) -> Self {
        self.font_selection = font_selection.into();
        self
    }

    pub fn show(self, ui: &mut Ui, current: &mut E) -> Response {
        let font_id = self.font_selection.resolve(ui.style());
        for (label, selected_value) in self.selectable {
            let checked = selected_value == *current;
            let text = label.into().font(font_id.clone());
            let mut response = SelectableLabel::new(checked, text).ui(ui);

            if response.clicked() && *current != selected_value {
                *current = selected_value;
                response.mark_changed();
            }
        }

        let (_, response) = ui.allocate_at_least(Vec2::ZERO, Sense::click());
        response
    }
}
