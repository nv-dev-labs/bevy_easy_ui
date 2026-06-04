use bevy::prelude::*;

use crate::core::{image_node::EasyImageNode, node::EasyNode};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyImage {
    pub image_node: ImageNode,
    pub node: Node,
    pub border_color: BorderColor
}

#[derive(Default, Debug)]
pub struct EasyImageStyle {
    pub node: Node,
    pub border_color: BorderColor,
}

//>--------------------- IMPLEMENTATION ---------------------

impl EasyImage {
    pub fn new(image: Handle<Image>) -> EasyImage {
        EasyImage {
            image_node: ImageNode::new(image),
            node: Node {
                display: Display::Flex,
                ..default()
            },
            border_color: BorderColor::default()
        }
    }

    pub fn with_style(mut self, style: EasyImageStyle) -> Self {
        self.node = style.node;
        self.border_color = style.border_color;
        self
    }

    pub fn with_border_color(mut self, border_color: Color) -> Self {
        self.border_color = BorderColor::all(border_color);
        self
    }
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

//>--------------------- HELPERS --------------------------

impl std::ops::Deref for EasyImageStyle {
    type Target = Node;
    fn deref(&self) -> &Self::Target { &self.node }
}

impl From<EasyImage> for (ImageNode, Node, BorderColor) {
    fn from(image: EasyImage) -> (ImageNode, Node, BorderColor) {
       (image.image_node, image.node, image.border_color)
    }
}
