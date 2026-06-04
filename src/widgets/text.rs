use bevy::prelude::*;

use crate::core::{
    node::EasyNode,
    parts::{
        EasyStyle, EasyStyleExt, EasyTextProps, EasyTextPropsExt
    },
};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyText {
    pub text: Text,
    pub node: Node,
    pub style: EasyStyle,
    pub props: EasyTextProps,
}

#[derive(Default, Debug)]
pub struct EasyTextStyle {
    pub node: Node,
    pub style: EasyStyle,
    pub props: EasyTextProps,
}

//>--------------------- ACCESSOR IMPLS ---------------------

impl EasyNode for EasyText {
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl EasyStyleExt for EasyText {
    fn easy_style_mut(&mut self) -> &mut EasyStyle { &mut self.style }
}

impl EasyTextPropsExt for EasyText {
    fn easy_props_mut(&mut self) -> &mut EasyTextProps { &mut self.props }
}

//>--------------------- BUILDER API ---------------------

impl EasyText {
    pub fn new(text: &str) -> Self {
        EasyText {
            text: Text::new(text),
            node: Node::default(),
            style: EasyStyle::default(),
            props: EasyTextProps::default(),
        }
    }

    pub fn with_style(mut self, style: EasyTextStyle) -> Self {
        self.node = style.node;
        self.props.text_font = style.props.text_font;
        self.props.text_color = style.props.text_color;
        self.props.text_layout = style.props.text_layout;
        self.props.line_height = style.props.line_height;
        self.props.text_shadow = style.props.text_shadow;
        self.style = style.style;
        self
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = Text::new(text);
        self
    }
}
