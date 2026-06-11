use bevy_easy_ui::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SliderPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn update_slider(new_value: On<ValueChange<f32>>) {
    println!("Slider value changed: {}", new_value.value);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    EasyVerticalLayout::new()
        .with_width(percent(100.))
        .with_height(percent(100.))
        .with_justify_content(JustifyContent::Center)
        .with_align_items(AlignItems::Center)
        // .with_child(
            // EasySlider::new()
            //     .with_width(px(200.))
            //     .with_height(px(20.))
            //     .with_observer(update_slider),
        // )
        .spawn(&mut commands);
}