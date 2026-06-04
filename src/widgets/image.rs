use bevy::prelude::*;

use crate::core::{image_node::EasyImageNode, node::EasyNode};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyImage (
    ImageNode,
    Node,
    BorderColor
);

#[derive(Default, Debug)]
pub struct EasyImageStyle {
    pub node: Node,
    pub border_color: BorderColor,
}

//>--------------------- IMPLEMENTATION ---------------------

impl EasyImage {
    pub fn new(image: Handle<Image>) -> EasyImage {
        EasyImage(ImageNode::new(image), Node {
            display: Display::Flex,
            ..default()
        }, BorderColor::default())
    }

    pub fn with_style(mut self, style: EasyImageStyle) -> Self {
        self.1 = style.node;
        self.2 = style.border_color;
        self
    }

    pub fn with_border_color(mut self, border_color: Color) -> Self {
        self.2 = BorderColor::all(border_color);
        self
    }
}

impl EasyImageNode for EasyImage {
    fn node_mut(&mut self) -> &mut ImageNode {
        &mut self.0
    }
}

impl EasyNode for EasyImage {
    fn node_mut(&mut self) -> &mut Node {
        &mut self.1
    }
}

//>--------------------- HELPERS --------------------------

impl std::ops::Deref for EasyImageStyle {
    type Target = Node;
    fn deref(&self) -> &Self::Target { &self.node }
}

impl From<EasyImage> for (ImageNode, Node, BorderColor) {
    fn from(image: EasyImage) -> (ImageNode, Node, BorderColor) {
       (image.0, image.1, image.2)
    }
}
