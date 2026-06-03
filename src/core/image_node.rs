use bevy::{
    asset::Handle, color::Color, image::{
        Image,
        TextureAtlas
    }, math::Rect, ui::widget::{ImageNode, NodeImageMode}
};

#[allow(dead_code)]
pub trait EasyImageNode: Sized {
    fn node_mut(&mut self) -> &mut ImageNode;

    fn color(mut self, color: Color) -> Self {
        self.node_mut().color = color.into();
        self
    }

    fn image(mut self, image: Handle<Image>) -> Self {
        self.node_mut().image = image;
        self
    }

    fn texture_atlas(mut self, texture_atlas: TextureAtlas) -> Self {
        self.node_mut().texture_atlas = Some(texture_atlas);
        self
    }

    fn flip_x(mut self, flip_x: bool) -> Self {
        self.node_mut().flip_x = flip_x;
        self
    }

    fn flip_y(mut self, flip_y: bool) -> Self {
        self.node_mut().flip_y = flip_y;
        self
    }

    fn rect(mut self, rect: Rect) -> Self {
        self.node_mut().rect = Some(rect);
        self
    }

    fn image_mode(mut self, image_mode: NodeImageMode) -> Self {
        self.node_mut().image_mode = image_mode;
        self
    }
}
