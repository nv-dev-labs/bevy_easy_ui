use bevy::prelude::*;

use crate::{core::container::Container, widgets::{button::EasyButtonContainer, horizontal_layout::EasyHorizontalLayoutContainer, image::EasyImage, rich_text::EasyRichTextContainer, span::EasySpan, text::EasyText, vertical_layout::EasyVerticalLayoutContainer}};

//>--------------------- STRUCTURES ---------------------

pub enum EasyElement {
    // Containers (i.e. elements that can have children):
    ButtonContainer(EasyButtonContainer),
    TextContainer(EasyRichTextContainer),
    VerticalContainer(EasyVerticalLayoutContainer),
    HorizontalContainer(EasyHorizontalLayoutContainer),

    // Non-containers (i.e. leaf nodes):
    Image(EasyImage),
    Text(EasyText),
    Span(EasySpan),
}

//>--------------------- IMPLEMENTATIONS ---------------------

impl From<EasyButtonContainer> for EasyElement {
    fn from(b: EasyButtonContainer) -> Self { EasyElement::ButtonContainer(b) }
}
impl From<EasyRichTextContainer> for EasyElement {
    fn from(t: EasyRichTextContainer) -> Self { EasyElement::TextContainer(t) }
}
impl From<EasyVerticalLayoutContainer> for EasyElement {
    fn from(c: EasyVerticalLayoutContainer) -> Self {
        EasyElement::VerticalContainer(c)
    }
}
impl From<EasyHorizontalLayoutContainer> for EasyElement {
    fn from(c: EasyHorizontalLayoutContainer) -> Self {
        EasyElement::HorizontalContainer(c)
    }
}
impl From<EasyImage> for EasyElement {
    fn from(i: EasyImage) -> Self { EasyElement::Image(i) }
}
impl From<EasyText> for EasyElement {
    fn from(t: EasyText) -> Self { EasyElement::Text(t) }
}
impl From<EasySpan> for EasyElement {
    fn from(s: EasySpan) -> Self { EasyElement::Span(s) }
}

impl EasyElement {
    pub fn spawn_in(self, p: &mut ChildSpawnerCommands) {
        match self {
            // Button is a Container<EasyButton>: we spawn the bundle, then
            // recursively descend into its children (e.g. the label text).
            EasyElement::ButtonContainer(mut b) => {
                let entity = p.spawn(b.take_bundle()).id();
                let kids = b.take_children();
                p.commands().entity(entity).with_children(|sub| {
                    for child in kids {
                        child.spawn_in(sub);
                    }
                });
                for observer in b.take_observers() {
                    p.commands().spawn(observer.with_entity(entity));
                }
            }
            EasyElement::TextContainer(mut t) => {
                let entity = p.spawn(t.take_bundle()).id();
                let kids = t.take_children();
                p.commands().entity(entity).with_children(|sub| {
                    for child in kids {
                        // EasySpan is a leaf bundle, spawn it directly.
                        sub.spawn(child);
                    }
                });
                for observer in t.take_observers() {
                    p.commands().spawn(observer.with_entity(entity));
                }
            }
            EasyElement::VerticalContainer(mut c) => {
                let entity = p.spawn(c.take_bundle()).id();
                let kids = c.take_children();
                p.commands().entity(entity).with_children(|sub| {
                    for child in kids {
                        child.spawn_in(sub);
                    }
                });
                for observer in c.take_observers() {
                    p.commands().spawn(observer.with_entity(entity));
                }
            }
            EasyElement::HorizontalContainer(mut c) => {
                let entity = p.spawn(c.take_bundle()).id();
                let kids = c.take_children();
                p.commands().entity(entity).with_children(|sub| {
                    for child in kids {
                        child.spawn_in(sub);
                    }
                });
                for observer in c.take_observers() {
                    p.commands().spawn(observer.with_entity(entity));
                }
            }
            EasyElement::Image(i) => {
                p.spawn(i);
            },
            EasyElement::Text(t) => {
                p.spawn(t);
            },
            EasyElement::Span(s) => {
                p.spawn(s);
            }
        }
    }
}
