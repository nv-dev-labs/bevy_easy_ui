use bevy::prelude::*;

use crate::core::{image_node::EasyImageNode, node::EasyNode, parts::{EasyStyle, EasyStyleExt}};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyImage {
    pub image_node: ImageNode,
    pub node: Node,
    pub box_style: EasyStyle,
}

#[derive(Default, Debug)]
pub struct EasyImageStyle {
    pub node: Node,
    pub box_style: EasyStyle,
}

//>--------------------- ACCESSOR IMPLS ---------------------

impl EasyStyleExt for EasyImage {
    fn easy_style_mut(&mut self) -> &mut EasyStyle { &mut self.box_style }
}

impl EasyImageNode for EasyImage {
    fn node_mut(&mut self) -> &mut ImageNode {
        &mut self.image_node
    }
}

impl EasyNode for EasyImage {
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

//>--------------------- BUILDER API ---------------------

impl EasyImage {
    pub fn new(image: Handle<Image>) -> EasyImage {
        EasyImage {
            image_node: ImageNode::new(image),
            node: Node {
                display: Display::Flex,
                ..default()
            },
            box_style: EasyStyle::default(),
        }
    }

    pub fn with_style(mut self, style: EasyImageStyle) -> Self {
        self.node = style.node;
        self.box_style = EasyStyle {
            box_shadow: style.box_style.box_shadow,
            background_color: style.box_style.background_color,
            border_color: style.box_style.border_color,
        };
        self
    }
}
