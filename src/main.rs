use bevy::prelude::*;
use bevy_easy_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn hover_in(hover_in: On<Pointer<Over>>, mut commands: Commands) {
    commands.entity(hover_in.entity).insert(BackgroundColor(EasyColor::DARK_BLUE));
}

fn hover_out(hover_out: On<Pointer<Out>>, mut commands: Commands) {
    commands.entity(hover_out.entity).insert(BackgroundColor(EasyColor::BLACK));
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    EasyVerticalLayout::new()
        .with_width(percent(100.))
        .with_height(percent(100.))
        .with_justify_content(JustifyContent::Center)
        .with_align_items(AlignItems::Center)
        .with_child(
            // Vertical layout test
            EasyVerticalLayout::new()
                .with_align_items(AlignItems::Center)
                .with_background_color(EasyColor::BLUE)
                .with_padding(px(10.), px(10.), px(10.), px(10.))
                .with_margin(px(0.), px(0.), px(20.), px(0.))
                .with_child(
                    EasyButton::new()
                        .with_border_color(EasyColor::WHITE)
                        .with_border(px(2.), px(10.))
                        .with_margin(px(0.), px(0.), px(20.), px(0.))
                        .with_width(px(200.))
                        .with_height(px(80.))
                        .with_background_color(EasyColor::BLACK)
                        .with_observer(hover_in)
                        .with_observer(hover_out)
                        .with_child(
                            EasyText::new("Click me!")
                                .with_color(EasyColor::WHITE)
                                .with_font_size(24.)
                        )
                )
                .with_child(
                    EasyImage::new(asset_server.load("bevy.png"))
                        .with_flip_x(true)
                        .with_width(px(50.))
                        .with_height(px(50.))
                )
        )
        .with_child(
            // Horizontal layout test
            EasyHorizontalLayout::new()
                .with_border(px(2.), px(0.))
                .with_border_color(EasyColor::RED)
                .with_justify_content(JustifyContent::Center)
                .with_margin(px(0.), px(0.), px(20.), px(0.))
                .with_padding(px(10.), px(10.), px(10.), px(10.))
                .with_align_items(AlignItems::Center)
                .with_child(
                    EasyButton::new()
                        .with_border_color(EasyColor::WHITE)
                        .with_border(px(2.), px(10.))
                        .with_width(px(150.))
                        .with_height(px(60.))
                        .with_background_color(EasyColor::BLACK)
                        .with_observer(hover_in)
                        .with_observer(hover_out)
                        .with_child(
                            EasyLabel::new("To the left !")
                                .with_color(EasyColor::WHITE)
                                .with_font_size(18.)
                        )
                )
                .with_child(
                    EasyText::new("To the right !")
                        .with_color(EasyColor::WHITE)
                        .with_font_size(18.)
                )
        )
        .with_child(
            // Rich text test
            EasyRichText::new()
                .with_color(EasyColor::BLUE)
                .with_child(
                    EasySpan::new("This is a rich text widget. ")
                )
                .with_child(
                    EasySpan::new("I'm in red. ")
                        .with_color(EasyColor::RED)
                )
                .with_child(
                    EasySpan::new("And I'm in green. ")
                        .with_color(EasyColor::GREEN)
                )
                .with_child(
                    // ! Throws a compile-time error if changed to something else, e.g. `EasyText::new("Not a span")`
                    EasySpan::new("Only EasySpan is accepted in EasyRichText.")
                )
        )
        .spawn(&mut commands);
}
