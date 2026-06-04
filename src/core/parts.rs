use bevy::{
    asset::Handle,
    image::Image,
    math::Vec2,
    prelude::*,
    text::{
        Font, FontFeatures, FontSmoothing, FontWeight, Justify, LineBreak, LineHeight,
        TextColor, TextFont, TextLayout,
    },
    ui::widget::TextShadow,
};

//>===========================================================================
// BUNDLE PARTS (reusable, auto-flattened by `#[derive(Bundle)]`)
//>===========================================================================
//
// Bevy 0.18's `#[derive(Bundle)]` automatically flattens any field that itself
// implements `Bundle`. So a widget can simply embed these structs as fields
// — no attribute needed.
//
// IMPORTANT: Bevy's `Text` already requires `TextFont`, `TextColor`,
// `TextLayout`, `LineHeight` via `#[require(...)]`. So `EasyTextProps`
// cannot include them (would create duplicates when flattened alongside `Text`).
// It only contains `TextShadow` (not required by `Text`).

// === Common visual styling (background, border, shadow) ===
// Used by all widgets (text or not).
#[derive(Bundle, Default, Debug, Clone)]
pub struct EasyStyle {
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub box_shadow: BoxShadow,
}

// === Text-specific extra props (only what Text does NOT require) ===
// Used by text-bearing widgets (Text, Label, Span, RichText).
#[derive(Bundle, Default, Debug, Clone)]
pub struct EasyTextProps {
    pub text_shadow: TextShadow,
    pub text_font: TextFont,
    pub text_color: TextColor,
    pub text_layout: TextLayout,
    pub line_height: LineHeight,
}

// === Span-specific props (TextSpan doesn't require anything) ===
// Used by EasySpan only — TextSpan has no `#[require]` so it can hold
// font/color/layout directly. Plus visual styling.
#[derive(Bundle, Default, Debug, Clone)]
pub struct EasySpanProps {
    pub text_font: TextFont,
    pub text_color: TextColor,
    pub text_shadow: TextShadow,
    pub text_layout: TextLayout,
    pub line_height: LineHeight,
    pub box_shadow: BoxShadow,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
}

// === Image-specific props (extra styling beyond what EasyImage has) ===
#[derive(Bundle, Default, Debug, Clone)]
pub struct EasyImageProps {
    pub box_shadow: BoxShadow,
    pub background_color: BackgroundColor,
}

//>===========================================================================
// TEXT-REQUIRED FIELDS (used as direct fields, not as a Bundle part)
//>===========================================================================
//
// Since `Text` requires `TextFont`, `TextColor`, `TextLayout`, `LineHeight`,
// text widgets must declare them as direct fields. To keep the setter logic
// DRY, we expose a trait-based accessor system: each widget points to its
// own fields, and the setter is defined ONCE in the extension trait below.
//
// (Note: we could put these in a Bundle part too, but the require-vs-flatten
// conflict forces us to either duplicate or use field-level accessors.)
//
// For widgets that don't have these (button, layouts, image), they only
// implement `HasEasyStyle` and get the visual setters.

//>===========================================================================
// EXTENSION TRAITS (setters, defined once, applied via blanket impl)
//>===========================================================================

// === EasyStyleExt (auto-applied to any widget embedding `EasyStyle`) ===
// The widget points to its embedded `EasyStyle` via `easy_style_mut`, and we
// mutate the targeted field directly. The `..default()` fallback handles
// types like `BoxShadow` that aren't `Copy`.
pub trait EasyStyleExt: Sized {
    fn easy_style_mut(&mut self) -> &mut EasyStyle;
    
    fn with_background_color(mut self, color: Color) -> Self {
        self.easy_style_mut().background_color = BackgroundColor(color);
        self
    }
    
    fn with_border_color(mut self, color: Color) -> Self {
        self.easy_style_mut().border_color = BorderColor::all(color);
        self
    }
    
    fn with_box_shadow(mut self, box_shadow: BoxShadow) -> Self {
        self.easy_style_mut().box_shadow = box_shadow;
        self
    }
}

// === EasyTextPropsExt (for text-bearing widgets) ===
pub trait EasyTextPropsExt: Sized {
    fn easy_props_mut(&mut self) -> &mut EasyTextProps;

    fn with_color(mut self, text_color: Color) -> Self {
        self.easy_props_mut().text_color = TextColor(text_color);
        self
    }
    
    fn with_text_shadow(mut self, text_shadow: TextShadow) -> Self {
        self.easy_props_mut().text_shadow = text_shadow;
        self
    }
    
    fn with_shadow(mut self, text_shadow: TextShadow) -> Self {
        self.easy_props_mut().text_shadow = text_shadow;
        self
    }
    
    fn with_font_family(mut self, font: Handle<Font>) -> Self {
        self.easy_props_mut().text_font.font = font;
        self
    }
    
    fn with_font_size(mut self, font_size: f32) -> Self {
        self.easy_props_mut().text_font.font_size = font_size;
        self
    }
    
    fn with_font_weight(mut self, font_weight: FontWeight) -> Self {
        self.easy_props_mut().text_font.weight = font_weight;
        self
    }
    
    fn with_smoothing(mut self, font_smoothing: FontSmoothing) -> Self {
        self.easy_props_mut().text_font.font_smoothing = font_smoothing;
        self
    }
    
    fn with_features(mut self, font_features: FontFeatures) -> Self {
        self.easy_props_mut().text_font.font_features = font_features;
        self
    }
    fn with_justify(mut self, justify: Justify) -> Self {
        self.easy_props_mut().text_layout.justify = justify;
        self
    }
    
    fn with_linebreak(mut self, linebreak: LineBreak) -> Self {
        self.easy_props_mut().text_layout.linebreak = linebreak;
        self
    }
    
    fn with_line_height(mut self, line_height: LineHeight) -> Self {
        self.easy_props_mut().line_height = line_height;
        self
    }
}

// Suppress unused import warnings for items used in trait signatures.
#[allow(dead_code)]
fn _unused_imports(_: Image, _: Vec2) {}
