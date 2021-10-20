use bevy::{prelude::*, window::WindowMode};
//use bevy_mod_debug_console::ConsoleDebugPlugin;

mod plugin;

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.392, 0.584, 0.929))) // Cornflower Blue
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Test".to_string(),
            vsync: true,
            cursor_visible: true,
            mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        //.add_plugin(ConsoleDebugPlugin)
        .add_plugin(crate::plugin::ControlsPlugin {})
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let controls = gerg_ui::instantiate_controls_from_file("screen1.ui");
    let _entities = gerg_ui::spawn_controls(&mut commands, asset_server, materials, controls, Vec2::new(1920.0, 1080.0));
}
