use bevy::{prelude::*, text::{FontFeatures, FontSmoothing}};

use crate::core::node::EasyNode;

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyText {
    pub text: Text,
    pub node: Node,
    pub text_font: TextFont,
    pub text_color: TextColor,
    pub text_shadow: TextShadow,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
}

#[derive(Default, Debug)]
pub struct EasyTextStyle {
    pub node: Node,
    pub text_font: TextFont,
    pub text_color: TextColor,
    pub text_shadow: TextShadow,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
}

//>--------------------- IMPLEMENTATION ---------------------

impl EasyText {
    pub fn new(text: &str) -> Self {
        EasyText {
            text: Text::new(text),
            node: Node::default(),
            text_font: TextFont::default(),
            text_color: TextColor::default(),
            text_shadow: TextShadow::default(),
            background_color: BackgroundColor::default(),
            border_color: BorderColor::default()
        }
    }

    pub fn with_style(mut self, style: EasyTextStyle) -> Self {
        self.node = style.node;
        self.text_font = style.text_font;
        self.text_color = style.text_color;
        self.text_shadow = style.text_shadow;
        self.background_color = style.background_color;
        self.border_color = style.border_color;
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

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = Text::new(text);
        self
    }

    pub fn with_color(mut self, text_color: Color) -> Self {
        self.text_color = TextColor(text_color);
        self
    }

    pub fn with_text_shadow(mut self, text_shadow: TextShadow) -> Self {
        self.text_shadow = text_shadow;
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

impl EasyNode for EasyText {
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

//>--------------------- HELPERS ---------------------

impl std::ops::Deref for EasyTextStyle {
    type Target = Node;
    fn deref(&self) -> &Self::Target { &self.node }
}

impl From<EasyText> for (Text, Node, TextFont, TextColor, TextShadow, BackgroundColor, BorderColor) {
    fn from(text: EasyText) -> (Text, Node, TextFont, TextColor, TextShadow, BackgroundColor, BorderColor) {
       (text.text, text.node, text.text_font, text.text_color, text.text_shadow, text.background_color, text.border_color)
    }
}
