use eframe::{
    egui::{FontSelection, TextStyle},
    epaint::FontFamily,
};
use strum::AsRefStr;

#[derive(AsRefStr)]
pub enum CustomStyle {
    TextArea,
}

impl From<CustomStyle> for TextStyle {
    fn from(value: CustomStyle) -> Self {
        TextStyle::Name(value.as_ref().into())
    }
}

impl From<CustomStyle> for FontSelection {
    fn from(value: CustomStyle) -> Self {
        FontSelection::Style(value.into())
    }
}

#[derive(AsRefStr)]
pub enum CustomFont {
    TextArea,
}

impl From<CustomFont> for FontFamily {
    fn from(value: CustomFont) -> Self {
        FontFamily::Name(value.as_ref().into())
    }
}
