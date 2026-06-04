use bevy::prelude::*;

use crate::{
    core::{
        container::{
            Container,
            PushChild,
            PushObserver
        },
        node::EasyNode, parts::{EasyStyle, EasyStyleExt, EasyTextProps, EasyTextPropsExt}
    }, widgets::span::EasySpan
};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyRichText {
    pub text: Text,
    pub node: Node,
    pub box_style: EasyStyle,
    pub text_style: EasyTextProps,
}

pub struct EasyRichTextContainer {
    bundle: EasyRichText,
    children: Vec<EasySpan>,
    observers: Vec<Observer>,
}

#[derive(Default, Debug)]
pub struct EasyRichTextStyle {
    pub node: Node,
    pub box_style: EasyStyle,
    pub text_style: EasyTextProps,
}

//>--------------------- ACCESSOR IMPLS ---------------------

impl EasyStyleExt for EasyRichTextContainer {
    fn easy_style_mut(&mut self) -> &mut EasyStyle { &mut self.bundle.box_style }
}

impl EasyTextPropsExt for EasyRichTextContainer {
    fn easy_props_mut(&mut self) -> &mut EasyTextProps { &mut self.bundle.text_style }
}

impl EasyNode for EasyRichTextContainer {
    fn node_mut(&mut self) -> &mut Node {
        &mut self.bundle.node
    }
}

impl Container<EasySpan> for EasyRichTextContainer {
    fn take_bundle(&mut self) -> impl Bundle {
        std::mem::replace(&mut self.bundle, EasyRichText::default_bundle())
    }
    fn take_children(&mut self) -> Vec<EasySpan> {
        std::mem::take(&mut self.children)
    }
    fn take_observers(&mut self) -> Vec<Observer> { std::mem::take(&mut self.observers) }
}
impl PushChild<EasySpan> for EasyRichTextContainer {
    fn push_child(&mut self, c: EasySpan) {
       self.children.push(c);
    }
}
impl PushObserver<EasySpan> for EasyRichTextContainer {
    fn push_observer(&mut self, o: Observer) { self.observers.push(o); }
}

//>--------------------- BUILDER API ---------------------

impl EasyRichText {
    pub fn new() -> EasyRichTextContainer {
         EasyRichTextContainer {
            bundle: EasyRichText {
                text: Text::new(""),
                node: Node::default(),
                text_style: EasyTextProps::default(),
                box_style: EasyStyle::default(),
            },
            children: Vec::new(),
            observers: Vec::new(),
        }
    }

    fn default_bundle() -> Self {
        EasyRichText {
            text: Text::new(""),
            node: Node::default(),
            text_style: EasyTextProps::default(),
            box_style: EasyStyle::default(),
        }
    }
}

impl EasyRichTextContainer {
    pub fn with_style(mut self, style: EasyRichTextStyle) -> Self {
        self.bundle.node = style.node;
        self.bundle.box_style = style.box_style;
        self.bundle.text_style = style.text_style;
        self
    }
}
