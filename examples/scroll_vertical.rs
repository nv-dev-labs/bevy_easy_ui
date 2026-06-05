use bevy::prelude::*;
use bevy_easy_ui::prelude::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands) {
  commands.spawn(Camera2d);

  EasyVerticalLayout::new()
    .with_z_index(0)
    .with_background_color(EasyColor::DARK_GRAY)
    .with_width(percent(100.))
    .with_height(percent(100.))
    .with_justify_content(JustifyContent::Center)
    .with_align_items(AlignItems::Center)
    .with_padding(px(20.), px(20.), px(20.), px(20.))
    .with_child(
      EasyVerticalLayout::new()
        .with_z_index(1)
        .with_background_color(EasyColor::BLACK)
        .with_border_color(EasyColor::WHITE)
        .with_border(px(2.), px(8.))
        .with_width(px(320.))
        .with_height(px(300.))
        .with_overflow(Overflow::scroll())
        .with_scrollbar_width(8.0)
        .with_padding(px(10.), px(10.), px(10.), px(10.))
        .with_row_gap(px(6.))
        .with_align_items(AlignItems::Stretch)
        .with_child(
          EasyLabel::new("Scrollable list (try scrolling!)")
            .with_color(EasyColor::WHITE)
            .with_font_size(18.),
        )
        .with_child(EasyText::new("Item 1").with_color(EasyColor::LIGHT_GRAY))
        .with_child(EasyText::new("Item 2").with_color(EasyColor::LIGHT_GRAY))
        .with_child(EasyText::new("Item 3").with_color(EasyColor::LIGHT_GRAY))
        .with_child(EasyText::new("Item 4").with_color(EasyColor::LIGHT_GRAY))
        .with_child(EasyText::new("Item 5").with_color(EasyColor::LIGHT_GRAY))
        .with_child(EasyText::new("Item 6").with_color(EasyColor::LIGHT_GRAY))
        .with_child(EasyText::new("Item 7").with_color(EasyColor::LIGHT_GRAY))
        .with_child(EasyText::new("Item 8").with_color(EasyColor::LIGHT_GRAY))
        .with_child(EasyText::new("Item 9").with_color(EasyColor::LIGHT_GRAY))
        .with_child(EasyText::new("Item 10").with_color(EasyColor::LIGHT_GRAY))
        .with_child(EasyText::new("Item 11").with_color(EasyColor::LIGHT_GRAY))
        .with_child(EasyText::new("Item 12").with_color(EasyColor::LIGHT_GRAY))
        .with_child(EasyText::new("Item 13").with_color(EasyColor::LIGHT_GRAY))
        .with_child(EasyText::new("Item 14").with_color(EasyColor::LIGHT_GRAY))
        .with_child(EasyText::new("Item 15").with_color(EasyColor::LIGHT_GRAY))
        .with_child(EasyText::new("Item 16").with_color(EasyColor::LIGHT_GRAY))
        .with_child(EasyText::new("Item 17").with_color(EasyColor::LIGHT_GRAY))
        .with_child(EasyText::new("Item 18").with_color(EasyColor::LIGHT_GRAY))
        .with_child(EasyText::new("Item 19").with_color(EasyColor::LIGHT_GRAY))
        .with_child(EasyText::new("Item 20").with_color(EasyColor::LIGHT_GRAY)),
    )
    .spawn(&mut commands);
}
