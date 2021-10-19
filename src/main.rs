use bevy::{prelude::*, window::WindowMode};
use gerg_ui::ButtonState;
//use bevy_mod_debug_console::ConsoleDebugPlugin;

mod controls;
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
        //.add_plugin(crate::plugin::ControlsPlugin {})
        .add_startup_system(setup.system())
        .add_system(control_hover_system.system())
        .add_system(control_click_check_system.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let controls = gerg_ui::instantiate_controls_from_file("screen1.ui");
    let _entities = gerg_ui::spawn_controls(&mut commands, asset_server, materials, controls, Vec2::new(1920.0, 1080.0));
}

fn control_click_check_system(
    windows: Res<Windows>,
    mouse_input: Res<Input<MouseButton>>,
    mut control_query: Query<(&Sprite, &Transform, &mut Handle<ColorMaterial>, &mut gerg_ui::GergButton)>
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let cursor_position = get_cursor_position(windows);

        for (sprite, transform, mut color_material, mut button) in control_query.iter_mut() {
            let control_boundingbox = get_control_rectangle(sprite, transform);
    
            // if mouse is over control
            if cursor_position_overlaps_control_rect(&cursor_position, &control_boundingbox) {
                match button.button_state {
                    ButtonState::Hover => {
                        // change to active
                        button.button_state = ButtonState::Active;
                        *color_material = button.color_material_handle_active.clone()

                        // TODO: play sound
                        // TODO: start cooldown timer
                        // TODO: call some sort of func/action delegate
                    },
                    _ => { } // do nothing
                }
            }
        }
    }
}

fn control_hover_system(
    windows: Res<Windows>,
    mut control_query: Query<(&Sprite, &Transform, &mut Handle<ColorMaterial>, &mut gerg_ui::GergButton)>
) {
    let cursor_position = get_cursor_position(windows);

    for (sprite, transform, mut color_material, mut button) in control_query.iter_mut() {

        let control_boundingbox = get_control_rectangle(sprite, transform);

        // if mouse is over control
        if cursor_position_overlaps_control_rect(&cursor_position, &control_boundingbox) {
            match button.button_state {
                ButtonState::Normal => {
                    // change to hover
                    button.button_state = ButtonState::Hover;
                    *color_material = button.color_material_handle_hover.clone()
                },
                _ => { } // do nothing
            }
        } else {
            // else mouse is not over control
            match button.button_state {
                ButtonState::Hover => {
                    // change to normal
                    button.button_state = ButtonState::Normal;
                    *color_material = button.color_material_handle_normal.clone()
                },
                _ => { } // do nothing
            }
        }
    }
}

fn get_cursor_position(windows: Res<Windows>) -> Vec2 {
    let window = windows.get_primary().expect("no primary window");
    let screen_size = Vec2::new(window.width(), window.height());

    let cursor_position = match window.cursor_position() {
        Some(cp) => cp,
        None => Vec2::new(-1.0, -1.0),
    };

    let cursor_position = cursor_position - screen_size / 2.0;

    cursor_position
}

fn get_control_rectangle(sprite: &Sprite, transform: &Transform) -> Rect<f32> {
    let width = sprite.size.x;
    let height = sprite.size.y;
    let x = transform.translation.x - (width * 0.5);
    let y = transform.translation.y + (height * 0.5);
    let rect = Rect { left: x, right: x + width, top: y, bottom: y - height };
    rect
}

fn cursor_position_overlaps_control_rect(cursor_position: &Vec2, control_boundingbox: &Rect<f32>) -> bool {
    if cursor_position.x >= control_boundingbox.left && cursor_position.x <= control_boundingbox.right &&
       cursor_position.y >= control_boundingbox.bottom && cursor_position.y <= control_boundingbox.top {
        return true;
    } else {
        return false;
    }
}
