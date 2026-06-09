use bevy::prelude::*;

#[derive(Bundle, Default, Debug, Clone)]
pub struct EasyStackStyle {
  pub z_index: ZIndex,
  pub global_z_index: GlobalZIndex,
}

pub trait EasyStackStyleExt: Sized {
  fn easy_stack_style_mut(&mut self) -> &mut EasyStackStyle;

  fn with_z_index(mut self, z_index: i32) -> Self {
    self.easy_stack_style_mut().z_index = ZIndex(z_index);
    self
  }

  fn with_global_z_index(mut self, global_z_index: i32) -> Self {
    self.easy_stack_style_mut().global_z_index = GlobalZIndex(global_z_index);
    self
  }
}
