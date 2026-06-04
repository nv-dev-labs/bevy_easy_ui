use bevy::{ecs::bundle::Bundle, ui::{Node, widget::{Label, Text}}};

use crate::core::{node::EasyNode, parts::{EasyStyle, EasyStyleExt, EasyTextProps, EasyTextPropsExt}};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyLabel {
    pub text: Text,
    pub node: Node,
    pub style: EasyStyle,
    pub props: EasyTextProps,
    pub label: Label,
}

#[derive(Default, Debug)]
pub struct EasyLabelStyle {
    pub node: Node,
    pub style: EasyStyle,
    pub props: EasyTextProps,
}

//>--------------------- ACCESSOR IMPLS ---------------------

impl EasyStyleExt for EasyLabel {
    fn easy_style_mut(&mut self) -> &mut EasyStyle { &mut self.style }
}

impl EasyTextPropsExt for EasyLabel {
    fn easy_props_mut(&mut self) -> &mut EasyTextProps { &mut self.props }
}

impl EasyNode for EasyLabel {
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

//>--------------------- BUILDER API ---------------------

impl EasyLabel {
    pub fn new(text: &str) -> Self {
        EasyLabel {
            text: Text::new(text),
            node: Node::default(),
            style: EasyStyle::default(),
            props: EasyTextProps::default(),
            label: Label
        }
    }

    pub fn with_style(mut self, style: EasyLabelStyle) -> Self {
        self.node = style.node;
        self.style = EasyStyle { 
            background_color: style.style.background_color,
            border_color: style.style.border_color, 
            box_shadow: style.style.box_shadow 
        };
        self.props = EasyTextProps {
            text_color: style.props.text_color,
            text_shadow: style.props.text_shadow,
            text_font: style.props.text_font,
            text_layout: style.props.text_layout,
            line_height: style.props.line_height,
        };
        self
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = Text::new(text);
        self
    }
}


