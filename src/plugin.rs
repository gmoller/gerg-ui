use bevy::prelude::*;

use crate::shapes::Circle;

pub struct ControlsPlugin {}
impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(button_click_check_system.system())
            .add_system(button_hover_system.system())
            .add_system(button_cooldown_system.system())
            .add_system(destroy_controls_system.system());
    }
}

fn button_cooldown_system(
    mut commands: Commands,
    time: Res<Time>,
    mut control_query: Query<(Entity, &mut Handle<ColorMaterial>, &mut Cooldown, &mut GergButton)>
) {
    for (entity, mut color_material, mut cooldown, mut button) in control_query.iter_mut() {
        cooldown.remaining_time_in_seconds -= time.delta_seconds();

        if cooldown.remaining_time_in_seconds <= 0.0 {
            // change to normal
            button.button_state = ButtonState::Normal;
            *color_material = button.color_material_handle_normal.clone();

            commands.entity(entity).remove::<Cooldown>();
        }
    }
}

fn button_click_check_system(
    mut commands: Commands,
    windows: Res<Windows>,
    mouse_input: Res<Input<MouseButton>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut control_query: Query<(Entity, &Sprite, &Transform, &mut Handle<ColorMaterial>, &mut GergButton)>
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let cursor_position = get_cursor_position(windows);

        for (entity, sprite, transform, mut color_material, mut button) in control_query.iter_mut() {
            let control_bounding_shapes = get_control_bounding_shapes(button.bounding_box, button.bounding_circle, sprite, transform);
    
            // if mouse is over control
            let collision = cursor_position_overlaps_control_bounding_shapes(cursor_position, control_bounding_shapes);
            if collision {
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

                        commands.entity(entity).insert(Cooldown { remaining_time_in_seconds: 0.5 });
                        
                        // TODO: call some sort of func/action delegate
                        commands.entity(entity).insert(ButtonClicked);
                    },
                    _ => { } // do nothing
                }
            }
        }
    }
}

fn button_hover_system(
    windows: Res<Windows>,
    mut control_query: Query<(&Sprite, &Transform, &mut Handle<ColorMaterial>, &mut GergButton)>
) {
    let cursor_position = get_cursor_position(windows);

    for (sprite, transform, mut color_material, mut button) in control_query.iter_mut() {
        let control_bounding_shapes = get_control_bounding_shapes(button.bounding_box, button.bounding_circle, sprite, transform);

        // if mouse is over control
        let collision = cursor_position_overlaps_control_bounding_shapes(cursor_position, control_bounding_shapes);
        if collision {
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

fn destroy_controls_system(
    mut commands: Commands,
    controls_to_be_destroyed_query: Query<Entity, With<DestroyControl>>
) {
    for entity in  controls_to_be_destroyed_query.iter() {
        commands.entity(entity).despawn_recursive();
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

fn get_control_bounding_shapes(button_bounding_box: Vec4, button_bounding_circle: Vec3, sprite: &Sprite, transform: &Transform) -> (Option<Rect<f32>>, Option<Circle>) {
    if button_bounding_box.x == 0.0 && button_bounding_box.y == 0.0 && button_bounding_box.z == 0.0 && button_bounding_box.w == 0.0 {
        if button_bounding_circle.x == 0.0 && button_bounding_circle.y == 0.0 && button_bounding_circle.z == 0.0 {
            let rect = create_rectangle(button_bounding_box.x, button_bounding_box.y, sprite.size.x, sprite.size.y, transform);

            return (Some(rect), None);
        } else {
            let circle = create_circle(transform, button_bounding_circle);

            return (None, Some(circle));
        };
    } else {
        if button_bounding_circle.x == 0.0 && button_bounding_circle.y == 0.0 && button_bounding_circle.z == 0.0 {
            let rect = create_rectangle(button_bounding_box.x, button_bounding_box.y, button_bounding_box.z, button_bounding_box.w, transform);

            return (Some(rect), None);
        } else {
            let rect = create_rectangle(button_bounding_box.x, button_bounding_box.y, button_bounding_box.z, button_bounding_box.w, transform);
            let circle = create_circle(transform, button_bounding_circle);

            return (Some(rect), Some(circle));
        };
    };
}

fn create_rectangle(x: f32, y: f32, width: f32, height: f32, transform: &Transform) -> Rect<f32> {
    let x = (transform.translation.x + x) - (width * 0.5);
    let y = (transform.translation.y + y) + (height * 0.5);
    let rect = Rect { left: x, right: x + width, top: y, bottom: y - height };

    rect
}

fn create_circle(transform: &Transform, button_bounding_circle: Vec3) -> Circle {
    let center = Vec2::new(transform.translation.x + button_bounding_circle.x, transform.translation.y + button_bounding_circle.y);
    let radius = button_bounding_circle.z;
    let circle = Circle { center, radius };

    circle
}

fn cursor_position_overlaps_control_bounding_shapes(cursor_position: Vec2, bounding_shapes: (Option<Rect<f32>>, Option<Circle>)) -> bool {
    let rectangle_collides = match bounding_shapes.0 {
        Some(rect) => cursor_position_overlaps_control_rect(cursor_position, &rect),
        None => true,
    };

    let circle_collides = match bounding_shapes.1 {
        Some(circle) => cursor_position_overlaps_control_circle(cursor_position, &circle),
        None => true,
    };

    rectangle_collides && circle_collides
}

fn cursor_position_overlaps_control_rect(cursor_position: Vec2, control_bounding_box: &Rect<f32>) -> bool {
    let result = if cursor_position.x >= control_bounding_box.left && cursor_position.x <= control_bounding_box.right &&
       cursor_position.y >= control_bounding_box.bottom && cursor_position.y <= control_bounding_box.top {
        true
    } else {
        false
    };

    result
}

fn cursor_position_overlaps_control_circle(cursor_position: Vec2, control_bounding_circle: &Circle) -> bool {
    let distance = control_bounding_circle.center - cursor_position;
    let length = distance.length();
    let result = length <= control_bounding_circle.radius;

    result
}

pub struct Cooldown {
    pub remaining_time_in_seconds: f32
}

pub struct ButtonClicked;

pub struct DestroyControl;

pub struct GergControl {
    pub group_name: String
}

pub struct GergPictureBox {
    pub name: String
}

pub struct GergLabel {
    pub name: String
}

pub struct GergButton {
    pub name: String,
    pub button_state: ButtonState,
    pub color_material_handle_normal: Handle<ColorMaterial>,
    pub color_material_handle_hover: Handle<ColorMaterial>,
    pub color_material_handle_active: Handle<ColorMaterial>,
    pub color_material_handle_disabled: Handle<ColorMaterial>,
    pub on_click_sound: String,
    pub bounding_box: Vec4,
    pub bounding_circle: Vec3,
}

pub enum ButtonState {
    Normal,
    Hover,
    Active,
    Disabled
}
