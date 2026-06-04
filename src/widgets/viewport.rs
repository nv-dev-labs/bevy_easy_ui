use bevy::{
    ecs::{bundle::Bundle, entity::Entity},
    ui::{
        Node,
        widget::ViewportNode
    }
};

use crate::core::node::EasyNode;

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyViewport {
    pub node: Node,
    pub viewport: ViewportNode,
}

#[derive(Default, Debug)]
pub struct EasyViewportStyle {
    pub node: Node,
}

//>--------------------- IMPLEMENTATION ---------------------

impl EasyViewport {
    pub fn new(camera: Entity) -> Self {
        EasyViewport {
            node: Node::default(),
            viewport: ViewportNode::new(camera),
        }
    }

    pub fn with_style(mut self, style: EasyViewportStyle) -> Self {
        self.node = style.node;
        self
    }
}

impl EasyNode for EasyViewport {
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

//>--------------------- HELPERS --------------------------

impl From<EasyViewport> for (
    Node,
    ViewportNode
) {
    fn from(viewport: EasyViewport) -> (Node, ViewportNode) {
        (viewport.node, viewport.viewport)
    }
}


