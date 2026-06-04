use bevy::prelude::*;

use crate::{core::container::Container, widgets::{button::EasyButtonContainer, horizontal_layout::EasyHorizontalLayoutContainer, image::EasyImage, label::EasyLabel, rich_text::EasyRichTextContainer, span::EasySpan, text::EasyText, vertical_layout::EasyVerticalLayoutContainer}};

//>--------------------- STRUCTURES ---------------------

pub enum EasyElement {
    // Containers (i.e. elements that can have children):
    ButtonContainer(EasyButtonContainer),
    RichTextContainer(EasyRichTextContainer),
    VerticalContainer(EasyVerticalLayoutContainer),
    HorizontalContainer(EasyHorizontalLayoutContainer),

    // Non-containers (i.e. leaf nodes):
    Image(EasyImage),
    Text(EasyText),
    Label(EasyLabel),
    Span(EasySpan),
}

//>--------------------- IMPLEMENTATIONS ---------------------

// Impl for containers
impl From<EasyButtonContainer> for EasyElement {
    fn from(b: EasyButtonContainer) -> Self { EasyElement::ButtonContainer(b) }
}
impl From<EasyRichTextContainer> for EasyElement {
    fn from(t: EasyRichTextContainer) -> Self { EasyElement::RichTextContainer(t) }
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

// Impl for non-containers
impl From<EasyImage> for EasyElement {
    fn from(i: EasyImage) -> Self { EasyElement::Image(i) }
}
impl From<EasyText> for EasyElement {
    fn from(t: EasyText) -> Self { EasyElement::Text(t) }
}
impl From<EasyLabel> for EasyElement {
    fn from(l: EasyLabel) -> Self { EasyElement::Label(l) }
}
impl From<EasySpan> for EasyElement {
    fn from(s: EasySpan) -> Self { EasyElement::Span(s) }
}

impl EasyElement {
    /// Spawns this EasyElement in the world, as a child of the given parent. This is done by matching on the type of the element (container vs non-container) and calling the appropriate helper function to spawn it.
     pub fn spawn_in(self, p: &mut ChildSpawnerCommands) {
        match self {
            // Containers (use the generic helper)
            EasyElement::ButtonContainer(c)
                => spawn_container(c, p),
            EasyElement::RichTextContainer(c)
                => spawn_rich_text(c, p),
            EasyElement::VerticalContainer(c)
                => spawn_container(c, p),
            EasyElement::HorizontalContainer(c)
                => spawn_container(c, p),

            // Leaves
            EasyElement::Image(i)   => { p.spawn(i); }
            EasyElement::Text(t)    => { p.spawn(t); }
            EasyElement::Label(l)   => { p.spawn(l); }
            EasyElement::Span(s)    => { p.spawn(s); }
        }
    }
}


/// **Helper function** to spawn an EasyElement that is a container (i.e. can have children). 
/// It spawns the container itself, then recursively spawns its children, and finally spawns its observers.
fn spawn_container(mut c: impl Container<EasyElement>, p: &mut ChildSpawnerCommands) {
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

/// **Helper function** to spawn an EasyRichTextContainer, which is a special case because it has a different bundle and children type (EasySpan only instead of generic EasyElement).
fn spawn_rich_text(mut t: EasyRichTextContainer, p: &mut ChildSpawnerCommands) {
    let entity = p.spawn(t.take_bundle()).id();
    let kids = t.take_children();
    p.commands().entity(entity).with_children(|sub| {
        for child in kids {
            sub.spawn(child);
        }
    });
    for observer in t.take_observers() {
        p.commands().spawn(observer.with_entity(entity));
    }
}
