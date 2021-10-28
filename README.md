# UI Screens for BevyEngine

![version](https://img.shields.io/crates/v/gerg_ui)
![downloads](https://img.shields.io/crates/d/gerg_ui)

This is a little thing I put together for creating simple UI screens using the BevyEngine.
The idea is to define the screens in a sort of poor-man's markup and this crate will then
provide some functions to create the UI 'objects'.

For example, the following in a file will create a screen looking like:

![sample_picture](https://raw.githubusercontent.com/gmoller/gerg-ui/main/Capture.PNG)

with the code:
```sh
// start_system
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let controls = gerg_ui::instantiate_controls_from_file("screen1.ui");
    let _entities = gerg_ui::spawn_controls(&mut commands, asset_server, materials, controls, Vec2::new(1920.0, 1080.0), String::from("screen1.ui"));
}

// button_click_system
fn close_button_click_system(
    mut commands: Commands,
    button_clicked_query: Query<(Entity, &GergButton, &GergControl), With<ButtonClicked>>,
    all_controls_query: Query<(Entity, &GergControl)>
) {
    for (entity, button, control) in button_clicked_query.iter() {
        println!("Hey, a button was clicked! - {} - {}", button.name, control.group_name);

        commands.entity(entity).remove::<ButtonClicked>();

        if button.name == "close_button" {
            for (entity, control) in all_controls_query.iter() {
                if control.group_name == "screen1.ui" {
                    commands.entity(entity).insert(DestroyControl);
                }
            }
        }
    }
}
```

```sh
--global_settings--
font_name: CrimsonText-Regular.ttf
font_size: 30
color: 255;255;255 // WHITE
--end--

--picture_box--
name: frame1
texture_name: big_frame.png // mandatory
size: 1200;782              // mandatory
//center_position: 0;0      // middle of screen is 0;0, defaults to 0;0 if missing, but dock_with (and offset) will override
draw_order: 0               // defaults to 0 if missing
dock_with: screen.top_left<->this.top_left
offset: 10;-80
--end--

--picture_box--
name: heading
texture_name: big_heading.png
size: 1200;76
draw_order: 0.1
dock_with: frame1.top_middle<->this.bottom_middle
offset: 0;-1
--end--

--button--
name: close_button
texture_name_normal: close_button_n.png
texture_name_hover: close_button_h.png
texture_name_active: close_button_a.png
texture_name_disabled: close_button_n.png
on_click_sound: audio/mouse_click_1.mp3
size: 43;44
//bounding_box: 0;0;43;44
bounding_circle: 0;0;20
draw_order: 0.2
dock_with: heading.top_right<->this.top_right
offset: -7;-7
--end--

--picture_box--
name: panel_inner
texture_name: inner_frame.png
size: 556;740
draw_order: 0.3
dock_with: frame1.center_left<->this.center_left
offset: 20;0
color: BLUE
--end--

--label--
name: label3
size: 200;50
text_string: Test1
font_size: 50
color: CYAN
dock_with: panel_inner.top_left<->this.top_left
offset: 15;-15
--end--
```
