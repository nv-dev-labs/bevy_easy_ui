use bevy::{prelude::*, text::{FontFeatures, FontSmoothing}};

use crate::{
    core::{
        container::{
            Container,
            PushChild,
            PushObserver
        },
        node::EasyNode
    }, widgets::span::EasySpan,
};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyRichText (
    Text,
    Node,
    TextFont,
    TextColor,
    TextShadow,
);

pub struct EasyRichTextContainer {
    bundle: EasyRichText,
    children: Vec<EasySpan>,
    observers: Vec<Observer>,
}

#[derive(Default, Debug)]
pub struct EasyRichTextStyle {
    pub node: Node,
    pub text_font: TextFont,
    pub text_color: TextColor,
    pub text_shadow: TextShadow,
}

//>--------------------- IMPLEMENTATION ---------------------

impl EasyRichText {
    pub fn new() -> EasyRichTextContainer {
         EasyRichTextContainer {
            bundle: EasyRichText(Text::new(""), Node::default(), TextFont::default(), TextColor::default(), TextShadow::default()),
            children: Vec::new(),
            observers: Vec::new(),
        }
    }

    fn default_bundle() -> Self {
        EasyRichText(Text::new(""), Node::default(), TextFont::default(), TextColor::default(), TextShadow::default())
    }
}

impl EasyRichTextContainer {
    pub fn with_style(mut self, style: EasyRichTextStyle) -> Self {
        self.bundle.1 = style.node;
        self.bundle.2 = style.text_font;
        self.bundle.3 = style.text_color;
        self.bundle.4 = style.text_shadow;
        self
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.bundle.0 = Text::new(text);
        self
    }

    pub fn with_color(mut self, text_color: Color) -> Self {
        self.bundle.3 = TextColor(text_color);
        self
    }

    pub fn with_text_shadow(mut self, text_shadow: TextShadow) -> Self {
        self.bundle.4 = text_shadow;
        self
    }

    pub fn with_font_family(mut self, text_font: Handle<Font>) -> Self {
        self.bundle.2.font = text_font;
        self
    }

    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.bundle.2.font_size = font_size;
        self
    }

    pub fn with_font_weight(mut self, font_weight: FontWeight) -> Self {
        self.bundle.2.weight = font_weight;
        self
    }

    pub fn with_smoothing(mut self, font_smoothing: FontSmoothing) -> Self {
        self.bundle.2.font_smoothing = font_smoothing;
        self
    }

    pub fn with_features(mut self, font_features: FontFeatures) -> Self {
        self.bundle.2.font_features = font_features;
        self
    }
}

impl EasyNode for EasyRichTextContainer {
    fn node_mut(&mut self) -> &mut Node {
        &mut self.bundle.1
    }
}

impl Container<EasySpan> for EasyRichTextContainer {
    fn take_bundle(&mut self) -> impl Bundle {
        std::mem::replace(&mut self.bundle, EasyRichText::default_bundle())
    }
    fn take_children(&mut self) -> Vec<EasySpan> {
        std::mem::take(&mut self.children)
    }
    fn take_observers(&mut self) -> Vec<Observer> { std::mem::take(&mut self.observers) }
}
impl PushChild<EasySpan> for EasyRichTextContainer {
    fn push_child(&mut self, c: EasySpan) {
       self.children.push(c);
    }
}
impl PushObserver<EasySpan> for EasyRichTextContainer {
    fn push_observer(&mut self, o: Observer) { self.observers.push(o); }
}

//>--------------------- HELPERS ---------------------

impl std::ops::Deref for EasyRichTextStyle {
    type Target = Node;
    fn deref(&self) -> &Self::Target { &self.node }
}

impl From<EasyRichText> for (Text, Node, TextFont, TextColor, TextShadow) {
    fn from(text: EasyRichText) -> (Text, Node, TextFont, TextColor, TextShadow) {
       (text.0, text.1, text.2, text.3, text.4)
    }
}
