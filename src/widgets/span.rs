use bevy::prelude::*;

use crate::core::parts::{EasyStyle, EasyStyleExt, EasyTextProps, EasyTextPropsExt};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasySpan {
    pub text_span: TextSpan,
    pub style: EasyStyle,
    pub props: EasyTextProps,
}

#[derive(Default, Debug)]
pub struct EasySpanStyle {
    pub style: EasyStyle,
    pub props: EasyTextProps,
}

//>--------------------- ACCESSOR IMPLS ---------------------

impl EasyStyleExt for EasySpan {
    fn easy_style_mut(&mut self) -> &mut EasyStyle { &mut self.style }
}

impl EasyTextPropsExt for EasySpan {
    fn easy_props_mut(&mut self) -> &mut EasyTextProps { &mut self.props }
}

//>--------------------- BUILDER API ---------------------

impl EasySpan {
    pub fn new(text: &str) -> Self {
        EasySpan {
            text_span: TextSpan::new(text),
            style: EasyStyle::default(),
            props: EasyTextProps::default(),
        }
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text_span = TextSpan::new(text);
        self
    }

    pub fn with_style(mut self, style: EasySpanStyle) -> Self {
        self.style = EasyStyle {
            background_color: style.style.background_color,
            border_color: style.style.border_color,
            box_shadow: style.style.box_shadow,
        };

        self.props = EasyTextProps {
            text_color: style.props.text_color,
            text_shadow: style.props.text_shadow,
            text_font: style.props.text_font,
            text_layout: style.props.text_layout,
            line_height: style.props.line_height,
        };
        self
    }
}
