


use bevy::{prelude::*, text::{FontFeatures, FontSmoothing, LineHeight}};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasySpan {
    pub text_span: TextSpan,
    pub text_font: TextFont,
    pub text_color: TextColor,
    pub text_shadow: TextShadow,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub text_layout: TextLayout,
    pub line_height: LineHeight,
}

#[derive(Default, Debug)]
pub struct EasySpanStyle {
    pub text_font: TextFont,
    pub text_color: TextColor,
    pub text_shadow: TextShadow,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub text_layout: TextLayout,
    pub line_height: LineHeight,
}

//>--------------------- IMPLEMENTATION ---------------------

impl EasySpan {
    pub fn new(text: &str) -> Self {
        EasySpan {
            text_span: TextSpan::new(text),
            text_font: TextFont::default(),
            text_color: TextColor::default(),
            text_shadow: TextShadow::default(),
            background_color: BackgroundColor::default(),
            border_color: BorderColor::default(),
            text_layout: TextLayout::default(),
            line_height: LineHeight::default(),
        }
    }

    pub fn with_style(mut self, style: EasySpanStyle) -> Self {
        self.text_font = style.text_font;
        self.text_color = style.text_color;
        self.text_shadow = style.text_shadow;
        self.background_color = style.background_color;
        self.border_color = style.border_color;
        self.text_layout = style.text_layout;
        self.line_height = style.line_height;
        self
    }

    pub fn with_line_height(mut self, line_height: LineHeight) -> Self {
        self.line_height = line_height;
        self
    }

    pub fn with_justify(mut self, justify: Justify) -> Self {
        self.text_layout.justify = justify;
        self
    }

    pub fn with_linebreak(mut self, linebreak: LineBreak) -> Self {
        self.text_layout.linebreak = linebreak;
        self
    }

    pub fn with_background_color(mut self, background_color: Color) -> Self {
        self.background_color = BackgroundColor(background_color);
        self
    }

     pub fn with_border_color(mut self, border_color: Color) -> Self {
        self.border_color = BorderColor::all(border_color);
        self
    }

    pub fn with_shadow(mut self, text_shadow: TextShadow) -> Self {
        self.text_shadow = text_shadow;
        self
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text_span = TextSpan::new(text);
        self
    }

    pub fn with_color(mut self, text_color: Color) -> Self {
        self.text_color = TextColor(text_color);
        self
    }

    pub fn with_font_family(mut self, text_font: Handle<Font>) -> Self {
        self.text_font.font = text_font;
        self
    }

    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.text_font.font_size = font_size;
        self
    }

    pub fn with_font_weight(mut self, font_weight: FontWeight) -> Self {
        self.text_font.weight = font_weight;
        self
    }

    pub fn with_smoothing(mut self, font_smoothing: FontSmoothing) -> Self {
        self.text_font.font_smoothing = font_smoothing;
        self
    }

    pub fn with_features(mut self, font_features: FontFeatures) -> Self {
        self.text_font.font_features = font_features;
        self
    }
}

//>--------------------- HELPERS ---------------------

impl From<EasySpan> for (
    TextSpan,
    TextFont,
    TextColor,
    TextShadow,
    BackgroundColor,
    BorderColor,
    TextLayout,
    LineHeight
) {
    fn from(text: EasySpan) -> (
        TextSpan,
        TextFont,
        TextColor,
        TextShadow,
        BackgroundColor,
        BorderColor,
        TextLayout,
        LineHeight
    ) {
       (
            text.text_span,
            text.text_font,
            text.text_color,
            text.text_shadow,
            text.background_color,
            text.border_color,
            text.text_layout,
            text.line_height
        )
    }
}
