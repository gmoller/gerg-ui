// use bevy::prelude::*;

// pub struct ControlsPlugin {}
// impl Plugin for ControlsPlugin {
//     fn build(&self, app: &mut AppBuilder) {
//         app.add_system(control_hover_system.system());
//     }
// }

// fn control_hover_system(
//     mut commands: Commands,
//     windows: Res<Windows>,
//     control_query: Query<(&Sprite, &Transform, &GergButton)>,
// ) {
//     // where is the mouse?
//     let cursor_position = get_cursor_position(windows);
//     //println!("control_hover_system!");

//     for (sprite, transform, button) in control_query.iter() {

//         // if mouse is over control
//         //let a = sprite.size.x;
//         //let b = sprite.size.y;

//         //println!("width: {}, height: {}", a, b);

//         // else mouse is not over control

//         println!("Button!");

//     }
// }

// fn get_cursor_position(windows: Res<Windows>) -> Vec2 {
//     let window = windows.get_primary().expect("no primary window");
//     let cursor_position = match window.cursor_position() {
//         Some(cp) => cp,
//         None => Vec2::new(-1.0, -1.0),
//     };
//     //println!("Mouse pos: {}", cursor_position);

//     cursor_position
// }
