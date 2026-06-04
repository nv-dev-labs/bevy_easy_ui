


use bevy::{prelude::*, text::{FontFeatures, FontSmoothing}};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasySpan(TextSpan, TextFont, TextColor);

#[derive(Default, Debug)]
pub struct EasySpanStyle {
    pub text_font: TextFont,
    pub text_color: TextColor,
}

//>--------------------- IMPLEMENTATION ---------------------

impl EasySpan {
    pub fn new(text: &str) -> Self {
        EasySpan(TextSpan::new(text), TextFont::default(), TextColor::default())
    }

    pub fn with_style(mut self, style: EasySpanStyle) -> Self {
        self.1 = style.text_font;
        self.2 = style.text_color;
        self
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.0 = TextSpan::new(text);
        self
    }

    pub fn with_color(mut self, text_color: Color) -> Self {
        self.2 = TextColor(text_color);
        self
    }

    pub fn with_font_family(mut self, text_font: Handle<Font>) -> Self {
        self.1.font = text_font;
        self
    }

    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.1.font_size = font_size;
        self
    }

    pub fn with_font_weight(mut self, font_weight: FontWeight) -> Self {
        self.1.weight = font_weight;
        self
    }

    pub fn with_smoothing(mut self, font_smoothing: FontSmoothing) -> Self {
        self.1.font_smoothing = font_smoothing;
        self
    }

    pub fn with_features(mut self, font_features: FontFeatures) -> Self {
        self.1.font_features = font_features;
        self
    }
}

//>--------------------- HELPERS ---------------------

impl From<EasySpan> for (TextSpan, TextFont, TextColor) {
    fn from(text: EasySpan) -> (TextSpan, TextFont, TextColor) {
       (text.0, text.1, text.2)
    }
}
