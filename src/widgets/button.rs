use bevy::prelude::*;

use crate::core::{container::{Container, PushChild, PushObserver}, element::EasyElement, node::EasyNode};

#[derive(Bundle)]
pub struct EasyButton (
    Button,
    Node,
    BorderColor,
    BackgroundColor,
);

pub struct EasyButtonContainer {
    bundle: EasyButton,
    children: Vec<EasyElement>,
    observers: Vec<Observer>,
}

#[derive(Default)]
pub struct EasyButtonStyle {
    pub node: Node,
    pub border_color: BorderColor,
    pub background_color: BackgroundColor,
}

//>------------------------------------------

//? Implémentation de la partie "API de construction" d'EasyButton, qui nous permet de construire la définition déclarative du bouton (sans le spawn)
impl EasyButton {
    pub fn new() -> EasyButtonContainer {
         EasyButtonContainer {
            bundle: EasyButton(Button, Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            }, BorderColor::default(), BackgroundColor::default()),
            children: Vec::new(),
            observers: Vec::new(),
        }
    }

    fn default_inner() -> Self {
        EasyButton(Button, Node::default(), BorderColor::default(), BackgroundColor::default())
    }

    pub fn with_style(mut self, style: EasyButtonStyle) -> Self {
        self.1 = style.node;
        self.2 = style.border_color;
        self.3 = style.background_color;
        self
    }

    pub fn border_color(mut self, border_color: Color) -> Self {
        self.2 = BorderColor::all(border_color);
        self
    }

    pub fn background_color(mut self, background_color: Color) -> Self {
        self.3 = BackgroundColor(background_color);
        self
    }
}

//? Setters de style + EasyNode sur le container pour qu'on puisse chaîner
//? `.width(...)`, `.border_color(...)`, etc. directement après `EasyButton::new()`.
impl EasyButtonContainer {
    pub fn border_color(mut self, border_color: Color) -> Self {
        self.bundle.2 = BorderColor::all(border_color);
        self
    }

    pub fn background_color(mut self, background_color: Color) -> Self {
        self.bundle.3 = BackgroundColor(background_color);
        self
    }
}

impl EasyNode for EasyButtonContainer {
    fn node_mut(&mut self) -> &mut Node {
        &mut self.bundle.1
    }
}

//? Implémentation du trait Container pour EasyButtonContainer, qui nous permet de le traiter comme un élément de l'arbre UI avec des enfants et des observers
impl Container for EasyButtonContainer {
    fn take_bundle(&mut self) -> impl Bundle {
        std::mem::replace(&mut self.bundle, EasyButton::default_inner())
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

//>------------------------------------------

// This allows us to treat EasyButtonStyle as a Node when applying styles, while still keeping the border and background colors separate for easy access
impl std::ops::Deref for EasyButtonStyle {
    type Target = Node;
    fn deref(&self) -> &Self::Target { &self.node }
}

// This allows us to convert an EasyButton into the actual components needed to spawn it in Bevy
impl From<EasyButton> for (Button, Node, BorderColor, BackgroundColor,) {
    fn from(button: EasyButton) -> (Button, Node, BorderColor, BackgroundColor) {
       (button.0, button.1, button.2, button.3)
    }
}
