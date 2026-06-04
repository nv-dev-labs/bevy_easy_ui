use bevy::{
    ecs::{bundle::Bundle, entity::Entity},
    ui::{
        BackgroundColor, Node, widget::ViewportNode
    }
};

use crate::core::{node::EasyNode, parts::{EasyStyle, EasyStyleExt}};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyViewport {
    pub node: Node,
    pub viewport: ViewportNode,
    pub box_style: EasyStyle,
}

#[derive(Default, Debug)]
pub struct EasyViewportStyle {
    pub node: Node,
    pub box_style: EasyStyle,
}

//>--------------------- IMPLEMENTATION ---------------------

impl EasyStyleExt for EasyViewport {
    fn easy_style_mut(&mut self) -> &mut EasyStyle { &mut self.box_style }
}

impl EasyViewport {
    pub fn new(camera: Entity) -> Self {
        EasyViewport {
            node: Node::default(),
            viewport: ViewportNode::new(camera),
            box_style: EasyStyle::default(),
        }
    }

    pub fn with_style(mut self, style: EasyViewportStyle) -> Self {
        self.node = style.node;
        self.box_style = style.box_style;
        self
    }

    pub fn with_target_camera(mut self, camera: Entity) -> Self {
        self.viewport = ViewportNode::new(camera);
        self
    }
}

impl EasyNode for EasyViewport {
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}


