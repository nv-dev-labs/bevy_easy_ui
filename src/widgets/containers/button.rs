use bevy::prelude::*;

use crate::core::{container::{Container, PushChild, PushObserver}, element::EasyElement, node::EasyNode};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyButton {
    pub button: Button,
    pub node: Node,
    pub border_color: BorderColor,
    pub background_color: BackgroundColor,
    pub box_shadow: BoxShadow,
}

pub struct EasyButtonContainer {
    bundle: EasyButton,
    children: Vec<EasyElement>,
    observers: Vec<Observer>,
}

#[derive(Default, Debug)]
pub struct EasyButtonStyle {
    pub node: Node,
    pub border_color: BorderColor,
    pub background_color: BackgroundColor,
    pub box_shadow: BoxShadow,
}

//>--------------------- IMPLEMENTATION ---------------------

impl EasyButton {
    pub fn new() -> EasyButtonContainer {
         EasyButtonContainer {
            bundle: EasyButton {
                button: Button,
                node: Node {
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                border_color: BorderColor::default(),
                background_color: BackgroundColor::default(),
                box_shadow: BoxShadow::default(),
            },
            children: Vec::new(),
            observers: Vec::new(),
        }
    }

    fn default_bundle() -> Self {
        EasyButton {
            button: Button,
            node: Node::default(),
            border_color: BorderColor::default(),
            background_color: BackgroundColor::default(),
            box_shadow: BoxShadow::default(),
        }
    }
}

impl EasyButtonContainer {
    pub fn with_border_color(mut self, border_color: Color) -> Self {
        self.bundle.border_color = BorderColor::all(border_color);
        self
    }

    pub fn with_background_color(mut self, background_color: Color) -> Self {
        self.bundle.background_color = BackgroundColor(background_color);
        self
    }

    pub fn with_box_shadow(mut self, box_shadow: BoxShadow) -> Self {
        self.bundle.box_shadow = box_shadow;
        self
    }

    pub fn with_style(mut self, style: EasyButtonStyle) -> Self {
        self.bundle.node = style.node;
        self.bundle.border_color = style.border_color;
        self.bundle.background_color = style.background_color;
        self.bundle.box_shadow = style.box_shadow;
        self
    }
}

impl EasyNode for EasyButtonContainer {
    fn node_mut(&mut self) -> &mut Node {
        &mut self.bundle.node
    }
}

impl Container for EasyButtonContainer {
    fn take_bundle(&mut self) -> impl Bundle {
        std::mem::replace(&mut self.bundle, EasyButton::default_bundle())
    }
    fn take_children(&mut self) -> Vec<EasyElement> { std::mem::take(&mut self.children) }
    fn take_observers(&mut self) -> Vec<Observer> { std::mem::take(&mut self.observers) }
}
impl PushChild for EasyButtonContainer {
    fn push_child(&mut self, c: EasyElement) { self.children.push(c); }
}
impl PushObserver for EasyButtonContainer {
    fn push_observer(&mut self, o: Observer) { self.observers.push(o); }
}

//>--------------------- HELPERS ---------------------

// This allows us to convert an EasyButton into the actual components needed to spawn it in Bevy
impl From<EasyButton> for (
    Button,
    Node,
    BorderColor,
    BackgroundColor,
    BoxShadow
) {
    fn from(button: EasyButton) -> (
        Button,
        Node,
        BorderColor,
        BackgroundColor,
        BoxShadow
    ) {
        (
            button.button,
            button.node, 
            button.border_color, 
            button.background_color, 
            button.box_shadow
        )
    }
}
