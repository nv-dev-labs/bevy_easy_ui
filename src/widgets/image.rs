use bevy::prelude::*;

use crate::core::{image_node::EasyImageNode, node::EasyNode};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyImage {
    pub image_node: ImageNode,
    pub node: Node,
    pub border_color: BorderColor,
    pub box_shadow: BoxShadow,
}

#[derive(Default, Debug)]
pub struct EasyImageStyle {
    pub node: Node,
    pub border_color: BorderColor,
    pub box_shadow: BoxShadow,
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
            border_color: BorderColor::default(),
            box_shadow: BoxShadow::default(),
        }
    }

    pub fn with_style(mut self, style: EasyImageStyle) -> Self {
        self.node = style.node;
        self.border_color = style.border_color;
        self.box_shadow = style.box_shadow;
        self
    }

    pub fn with_border_color(mut self, border_color: Color) -> Self {
        self.border_color = BorderColor::all(border_color);
        self
    }

    pub fn with_box_shadow(mut self, box_shadow: BoxShadow) -> Self {
        self.box_shadow = box_shadow;
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

impl From<EasyImage> for (
    ImageNode,
    Node,
    BorderColor,
    BoxShadow
) {
    fn from(image: EasyImage) -> (
        ImageNode,
        Node,
        BorderColor,
        BoxShadow
    ) {
       (
        image.image_node,
        image.node,
        image.border_color,
        image.box_shadow
       )
    }
}
