use bevy::prelude::*;

pub struct ControlsPlugin {}
impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(control_click_check_system.system())
            .add_system(control_hover_system.system());
    }
}

fn control_click_check_system(
    mut commands: Commands,
    windows: Res<Windows>,
    mouse_input: Res<Input<MouseButton>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut control_query: Query<(Entity, &Sprite, &Transform, &mut Handle<ColorMaterial>, &mut GergButton)>
) {

    //println!("control_click_check_system");
    if mouse_input.just_pressed(MouseButton::Left) {
        let cursor_position = get_cursor_position(windows);

        for (entity, sprite, transform, mut color_material, mut button) in control_query.iter_mut() {
            let control_boundingbox = get_control_rectangle(sprite, transform);
    
            // if mouse is over control
            if cursor_position_overlaps_control_rect(&cursor_position, &control_boundingbox) {
                match button.button_state {
                    ButtonState::Hover => {
                        // change to active
                        button.button_state = ButtonState::Active;
                        *color_material = button.color_material_handle_active.clone();

                        let sound = &button.on_click_sound;
                        if !sound.is_empty() {
                            let sound_effect = asset_server.load(sound.as_str());
                            audio.play(sound_effect);
                        }

                        commands.entity(entity).insert(Cooldown { remaining_time_in_ms: 100.0 });
                        
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
    mut control_query: Query<(&Sprite, &Transform, &mut Handle<ColorMaterial>, &mut GergButton)>
) {

    //println!("control_hover_system");
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

pub struct Cooldown {
    pub remaining_time_in_ms: f32
}

pub struct GergPictureBox;

pub struct GergLabel;

pub struct GergButton {
    pub button_state: ButtonState,
    pub color_material_handle_normal: Handle<ColorMaterial>,
    pub color_material_handle_hover: Handle<ColorMaterial>,
    pub color_material_handle_active: Handle<ColorMaterial>,
    pub color_material_handle_disabled: Handle<ColorMaterial>,
    pub on_click_sound: String
}

pub enum ButtonState {
    Normal,
    Hover,
    Active,
    Disabled
}
