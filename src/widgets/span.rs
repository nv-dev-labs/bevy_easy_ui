use bevy::prelude::*;

use crate::core::parts::{EasyStyle, EasyStyleExt, EasyTextProps, EasyTextPropsExt};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasySpan {
    pub text_span: TextSpan,
    pub box_style: EasyStyle,
    pub text_style: EasyTextProps,
}

#[derive(Default, Debug)]
pub struct EasySpanStyle {
    pub box_style: EasyStyle,
    pub text_style: EasyTextProps,
}

//>--------------------- ACCESSOR IMPLS ---------------------

impl EasyStyleExt for EasySpan {
    fn easy_style_mut(&mut self) -> &mut EasyStyle { &mut self.box_style }
}

impl EasyTextPropsExt for EasySpan {
    fn easy_props_mut(&mut self) -> &mut EasyTextProps { &mut self.text_style }
}

//>--------------------- BUILDER API ---------------------

impl EasySpan {
    pub fn new(text: &str) -> Self {
        EasySpan {
            text_span: TextSpan::new(text),
            box_style: EasyStyle::default(),
            text_style: EasyTextProps::default(),
        }
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text_span = TextSpan::new(text);
        self
    }

    pub fn with_style(mut self, style: EasySpanStyle) -> Self {
        self.box_style = EasyStyle {
            background_color: style.box_style.background_color,
            border_color: style.box_style.border_color,
            box_shadow: style.box_style.box_shadow,
        };

        self.text_style = EasyTextProps {
            text_color: style.text_style.text_color,
            text_shadow: style.text_style.text_shadow,
            text_font: style.text_style.text_font,
            text_layout: style.text_style.text_layout,
            line_height: style.text_style.line_height,
        };
        self
    }
}
