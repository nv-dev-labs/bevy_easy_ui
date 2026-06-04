use bevy::{
    ecs::{bundle::Bundle, entity::Entity},
    ui::{
        BackgroundColor, Node, widget::ViewportNode
    }
};

use crate::core::node::EasyNode;

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyViewport {
    pub node: Node,
    pub viewport: ViewportNode,
    pub background_color: BackgroundColor,
}

#[derive(Default, Debug)]
pub struct EasyViewportStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
}

//>--------------------- IMPLEMENTATION ---------------------

impl EasyViewport {
    pub fn new(camera: Entity) -> Self {
        EasyViewport {
            node: Node::default(),
            viewport: ViewportNode::new(camera),
            background_color: BackgroundColor::default(),
        }
    }

    pub fn with_style(mut self, style: EasyViewportStyle) -> Self {
        self.node = style.node;
        self.background_color = style.background_color;
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

//>--------------------- HELPERS --------------------------

impl From<EasyViewport> for (
    Node,
    ViewportNode,
    BackgroundColor
) {
    fn from(viewport: EasyViewport) -> (
        Node,
        ViewportNode,
        BackgroundColor
    ) {
        (
            viewport.node,
            viewport.viewport,
            viewport.background_color
        )
    }
}


