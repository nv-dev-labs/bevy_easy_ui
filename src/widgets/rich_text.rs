use bevy::{prelude::*, text::{FontFeatures, FontSmoothing, LineHeight}};

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
pub struct EasyRichText {
    pub text: Text,
    pub node: Node,
    pub text_font: TextFont,
    pub text_color: TextColor,
    pub text_shadow: TextShadow,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub text_layout: TextLayout,
    pub line_height: LineHeight,
}

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
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub text_layout: TextLayout,
    pub line_height: LineHeight,
}

//>--------------------- IMPLEMENTATION ---------------------

impl EasyRichText {
    pub fn new() -> EasyRichTextContainer {
         EasyRichTextContainer {
            bundle: EasyRichText {
                text: Text::new(""),
                node: Node::default(),
                text_font: TextFont::default(),
                text_color: TextColor::default(),
                text_shadow: TextShadow::default(),
                background_color: BackgroundColor::default(),
                border_color: BorderColor::default(),
                text_layout: TextLayout::default(),
                line_height: LineHeight::default(),
            },
            children: Vec::new(),
            observers: Vec::new(),
        }
    }

    fn default_bundle() -> Self {
        EasyRichText {
            text: Text::new(""),
            node: Node::default(),
            text_font: TextFont::default(),
            text_color: TextColor::default(),
            text_shadow: TextShadow::default(),
            background_color: BackgroundColor::default(),
            border_color: BorderColor::default(),
            text_layout: TextLayout::default(),
            line_height: LineHeight::default(),
        }
    }
}

impl EasyRichTextContainer {
    pub fn with_style(mut self, style: EasyRichTextStyle) -> Self {
        self.bundle.node = style.node;
        self.bundle.text_font = style.text_font;
        self.bundle.text_color = style.text_color;
        self.bundle.text_shadow = style.text_shadow;
        self.bundle.background_color = style.background_color;
        self.bundle.border_color = style.border_color;
        self.bundle.text_layout = style.text_layout;
        self.bundle.line_height = style.line_height;
        self
    }

    pub fn with_line_height(mut self, line_height: LineHeight) -> Self {
        self.bundle.line_height = line_height;
        self
    }

    pub fn with_justify(mut self, justify: Justify) -> Self {
        self.bundle.text_layout.justify = justify;
        self
    }

    pub fn with_linebreak(mut self, linebreak: LineBreak) -> Self {
        self.bundle.text_layout.linebreak = linebreak;
        self
    }

    pub fn with_border_color(mut self, border_color: Color) -> Self {
        self.bundle.border_color = BorderColor::all(border_color);
        self
    }

    pub fn with_background_color(mut self, background_color: Color) -> Self {
        self.bundle.background_color = BackgroundColor(background_color);
        self
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.bundle.text = Text::new(text);
        self
    }

    pub fn with_color(mut self, text_color: Color) -> Self {
        self.bundle.text_color = TextColor(text_color);
        self
    }

    pub fn with_text_shadow(mut self, text_shadow: TextShadow) -> Self {
        self.bundle.text_shadow = text_shadow;
        self
    }

    pub fn with_font_family(mut self, text_font: Handle<Font>) -> Self {
        self.bundle.text_font.font = text_font;
        self
    }

    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.bundle.text_font.font_size = font_size;
        self
    }

    pub fn with_font_weight(mut self, font_weight: FontWeight) -> Self {
        self.bundle.text_font.weight = font_weight;
        self
    }

    pub fn with_smoothing(mut self, font_smoothing: FontSmoothing) -> Self {
        self.bundle.text_font.font_smoothing = font_smoothing;
        self
    }

    pub fn with_features(mut self, font_features: FontFeatures) -> Self {
        self.bundle.text_font.font_features = font_features;
        self
    }
}

impl EasyNode for EasyRichTextContainer {
    fn node_mut(&mut self) -> &mut Node {
        &mut self.bundle.node
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

impl From<EasyRichText> for (
    Text,
    Node,
    TextFont,
    TextColor,
    TextShadow,
    BackgroundColor,
    BorderColor,
    TextLayout,
    LineHeight
) {
    fn from(text: EasyRichText) -> (
        Text,
        Node,
        TextFont,
        TextColor,
        TextShadow,
        BackgroundColor,
        BorderColor,
        TextLayout,
        LineHeight
    ) {
       (
            text.text,
            text.node,
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
