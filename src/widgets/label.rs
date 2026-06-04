use bevy::{ecs::bundle::Bundle, ui::{Node, widget::{Label, Text}}};

use crate::core::{node::EasyNode, parts::{EasyStyle, EasyStyleExt, EasyTextProps, EasyTextPropsExt}};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyLabel {
    pub text: Text,
    pub node: Node,
    pub box_style: EasyStyle,
    pub text_style: EasyTextProps,
    pub label: Label,
}

#[derive(Default, Debug)]
pub struct EasyLabelStyle {
    pub node: Node,
    pub box_style: EasyStyle,
    pub text_style: EasyTextProps,
}

//>--------------------- ACCESSOR IMPLS ---------------------

impl EasyStyleExt for EasyLabel {
    fn easy_style_mut(&mut self) -> &mut EasyStyle { &mut self.box_style }
}

impl EasyTextPropsExt for EasyLabel {
    fn easy_props_mut(&mut self) -> &mut EasyTextProps { &mut self.text_style }
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
            box_style: EasyStyle::default(),
            text_style: EasyTextProps::default(),
            label: Label
        }
    }

    pub fn with_style(mut self, style: EasyLabelStyle) -> Self {
        self.node = style.node;
        self.box_style = EasyStyle { 
            background_color: style.box_style.background_color,
            border_color: style.box_style.border_color, 
            box_shadow: style.box_style.box_shadow 
        };
        self.text_style = EasyTextProps {
            text_color: style.text_style.text_color,
            text_shadow: style.text_style.text_shadow,
            text_font: style.text_style.text_font,
            text_layout: style.text_style.text_layout,
            line_height: style.text_style.line_height,
        };
        self
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = Text::new(text);
        self
    }
}


