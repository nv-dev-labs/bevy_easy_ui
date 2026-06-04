use bevy::{asset::Handle, color::Color, text::{Font, FontFeatures, FontSmoothing, FontWeight, TextColor, TextFont}, ui::{Node, widget::{Label, Text, TextShadow}}};

//>--------------------- STRUCTURES ---------------------

pub struct EasyLabel (
    Text,
    Node,
    TextFont,
    TextColor,
    TextShadow,
    Label
);

pub struct EasyLabelStyle {
    pub node: Node,
    pub text_font: TextFont,
    pub text_color: TextColor,
    pub text_shadow: TextShadow,
}

//>--------------------- IMPLEMENTATION ---------------------

impl EasyLabel {
    pub fn new(text: &str) -> Self {
        EasyLabel(
            Text::new(text),
            Node::default(),
            TextFont::default(),
            TextColor::default(),
            TextShadow::default(),
            Label
        )
    }

    pub fn with_style(mut self, style: EasyLabelStyle) -> Self {
        self.1 = style.node;
        self.2 = style.text_font;
        self.3 = style.text_color;
        self.4 = style.text_shadow;
        self
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.0 = Text::new(text);
        self
    }

    pub fn with_color(mut self, text_color: Color) -> Self {
        self.3 = TextColor(text_color);
        self
    }

    pub fn with_text_shadow(mut self, text_shadow: TextShadow) -> Self {
        self.4 = text_shadow;
        self
    }

    pub fn with_font_family(mut self, text_font: Handle<Font>) -> Self {
        self.2.font = text_font;
        self
    }

    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.2.font_size = font_size;
        self
    }

    pub fn with_font_weight(mut self, font_weight: FontWeight) -> Self {
        self.2.weight = font_weight;
        self
    }

    pub fn with_smoothing(mut self, font_smoothing: FontSmoothing) -> Self {
        self.2.font_smoothing = font_smoothing;
        self
    }

    pub fn with_features(mut self, font_features: FontFeatures) -> Self {
        self.2.font_features = font_features;
        self
    }
}

//>--------------------- HELPERS ---------------------

impl std::ops::Deref for EasyLabelStyle {
    type Target = Node;
    fn deref(&self) -> &Self::Target { &self.node }
}

impl From<EasyLabel> for (Text, Node, TextFont, TextColor, TextShadow, Label) {
    fn from(label: EasyLabel) -> (Text, Node, TextFont, TextColor, TextShadow, Label) {
       (label.0, label.1, label.2, label.3, label.4, label.5)
    }
}


