use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use bevy::prelude::*;

#[derive(Default)]
struct GlobalSettings {
    font_name: String,
    font_size: f32,
    color: Color
}

#[derive(Default)]
struct Control {
    control_type: ControlType,
    name: String,
    top_left_position: Vec2,
    size: Vec2,
    dock_with: String,
    offset: Vec2,

    // picture_box
    texture_name: String,
    draw_order: f32,

    // label
    text_string: String,
    font_name: String,
    font_size: f32,
    color: Color,
}

pub struct Controls {
    map: HashMap<String, Control>
}
impl Controls {
    fn get_by_name(&self, name: String) -> &Control {
        let foo = self.map.get(&name);
        let result = match foo {
            None => panic!("Control with name [{}] not found.", name),
            Some(control) => control
        };

        return result;
    }
}

enum ControlType {
    None,
    PictureBox,
    Label
}
impl Default for ControlType {
    fn default() -> Self { ControlType::None }
}

enum ReadState {
    None,
    GlobalSettings,
    PictureBox,
    Label
}

pub fn read_ui_file(ui_filename: &str) -> Vec<String> {

    let filename = format!("assets/{}", ui_filename);
    let filename_as_str = filename.as_str();
    let file = File::open(filename_as_str).expect(format!("Unable to open file [{}].", filename_as_str).as_str());
    let reader = BufReader::new(file);

    let mut list = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Unable to read line.");
        list.push(line);
    }

    return list;
}

pub fn instantiate_controls(lines: Vec<String>) -> Controls {

    let mut controls = HashMap::new();

    let mut read_state = ReadState::None;
    let mut line_number = 0;
    let mut global_settings = GlobalSettings { ..Default::default() };
    let mut control = Control { ..Default::default() };

    for line in lines {

        line_number += 1;
        if line.is_empty() { continue; } // skip line
        if line.starts_with("//") { continue; } // skip line

        match line.as_str() {
            "--global_settings--" => {
                read_state = ReadState::GlobalSettings;
                global_settings = GlobalSettings { ..Default::default() };
            },
            "--picture_box--" => {
                read_state = ReadState::PictureBox;
                control.control_type = ControlType::PictureBox;

            },
            "--label--" => {
                read_state = ReadState::Label;
                control.control_type = ControlType::Label;
                control.font_name = global_settings.font_name.clone();
                control.font_size = global_settings.font_size;
                control.color = global_settings.color;
            },
            "--end--" => {
                match read_state {
                    ReadState::None => { panic!("End found while not in a valid state. Line #{}: {}.", line_number, line); },
                    ReadState::GlobalSettings => { }, // do nothing
                    ReadState::PictureBox => {
                        controls.insert(control.name.clone(), control);
                    },
                    ReadState::Label => {
                        controls.insert(control.name.clone(), control);
                    }
                }
                read_state = ReadState::None;
                control = Control { ..Default::default() };
            },
            _ => {
                let split_str = line.split(':').collect::<Vec<&str>>();

                match read_state {
                    ReadState::None => { panic!("Not in a valid state. Line #{}: {}.", line_number, line); },
                    ReadState::GlobalSettings => {
                        match split_str[0] {
                            "font_name" => { global_settings.font_name = get_string(line); },
                            "font_size" => { global_settings.font_size = get_f32(line); },
                            "color" => { global_settings.color = get_color(line); },
                            _ => { panic!("Unknown field. Line#{}: {}.", line_number, line); }
                        }
                    },
                    ReadState::PictureBox => {
                        match split_str[0] {
                            "name" => { control.name = get_string(line); },
                            "center_position" => { let center_position = get_vec2(line); control.top_left_position = Vec2::new(center_position.x - control.size.x * 0.5, center_position.y + control.size.y * 0.5) },
                            "size" => { control.size = get_vec2(line); },
                            "dock_with" => { control.dock_with = get_string(line); },
                            "offset" => { control.offset = get_vec2(line); },

                            "texture_name" => { control.texture_name = get_string(line); },
                            "draw_order" => { control.draw_order = get_f32(line); },
                            _ => { panic!("Unknown field. Line#{}: {}.", line_number, line); }
                        }
                    },
                    ReadState::Label => {
                        match split_str[0] {
                            "name" => { control.name = get_string(line); },
                            "top_left_position" => { control.top_left_position = get_vec2(line); },
                            "size" => { control.size = get_vec2(line); },
                            "dock_with" => { control.dock_with = get_string(line); },
                            "offset" => { control.offset = get_vec2(line); },

                            "text_string" => { control.text_string = get_string(line); },
                            "font_name" => { control.font_name = get_string(line); },
                            "font_size" => { control.font_size = get_f32(line); },
                            "color" => { control.color = get_color(line); },
                            _ => { panic!("Unknown field. Line#{}: {}.", line_number, line); }
                        }
                    }
                }
            }
        }
    }

    let result = Controls { map: controls };

    return result;
}

pub fn spawn_controls(commands: &mut Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>, controls: Controls) {

    let controls_map = &controls.map;
    for (_, control) in controls_map {

        match control.control_type {
            ControlType::None => panic!("Can not spawn control [{}]. Unknown control_type.", control.name),
            ControlType::PictureBox => {
                let scale = Vec3::new(1.0, 1.0, 1.0);
                let material_path = control.texture_name.as_str();
                let texture_handle = asset_server.load(material_path);
                let material = materials.add(texture_handle.into());
            
                let top_left_position = calculate_top_left_position(control, &controls);
                let center_position = Vec3::new(top_left_position.x + control.size.x * 0.5, top_left_position.y - control.size.y * 0.5, control.draw_order);

                let bundle = instantiate_sprite_bundle(control.size, center_position, scale, material);
                commands.spawn_bundle(bundle);
            },
            ControlType::Label => {
                let min_size = Vec2::new(0.0, 0.0);
                let font_path = format!("fonts/{}", control.font_name);
                let font_handle: Handle<Font> = asset_server.load(font_path.as_str());
            
                let mut top_left_position = calculate_top_left_position(control, &controls);
                top_left_position.x += 1920.0 * 0.5;
                top_left_position.y = 1080.0 * 0.5 - top_left_position.y;

                let bundle = instantiate_textbundle(top_left_position, min_size, control.size, control.text_string.clone(), font_handle, control.font_size, control.color);
                commands.spawn_bundle(bundle);
            }
        }
    }
}

fn calculate_top_left_position(control: &Control, controls: &Controls) -> Vec2 {

    if control.dock_with.is_empty() {
        return control.top_left_position;
    }

    let split_str = control.dock_with.split("<->").collect::<Vec<&str>>(); // panel_inner.top_left<->this.top_left
    let left = split_str[0]; // panel_inner.top_left
    let right = split_str[1]; // this.top_left

    let left_split_str = left.split(".").collect::<Vec<&str>>();
    let control_to_use_for_docking = left_split_str[0]; // panel_inner
    let point_on_control_to_anchor_to = left_split_str[1]; // top_left
    let control_to_dock_to = controls.get_by_name(control_to_use_for_docking.to_string());

    let parent_top_left_position = calculate_top_left_position(control_to_dock_to, controls);
    //println!("Control: {}: parent_top_left_position: {}", control.name, parent_top_left_position);

    let pixel1 = match point_on_control_to_anchor_to {
        "top_left" => Vec2::new(parent_top_left_position.x, parent_top_left_position.y),
        "center_left" => Vec2::new(parent_top_left_position.x, parent_top_left_position.y - control_to_dock_to.size.y * 0.5),
        "bottom_left" => Vec2::new(parent_top_left_position.x, parent_top_left_position.y - control_to_dock_to.size.y),
        
        "top_middle" => Vec2::new(parent_top_left_position.x + control_to_dock_to.size.x * 0.5, parent_top_left_position.y),
        "center_middle" => Vec2::new(parent_top_left_position.x + control_to_dock_to.size.x * 0.5, parent_top_left_position.y - control_to_dock_to.size.y * 0.5),
        "bottom_middle" => Vec2::new(parent_top_left_position.x + control_to_dock_to.size.x * 0.5, parent_top_left_position.y - control_to_dock_to.size.y),

        "top_right" => Vec2::new(parent_top_left_position.x + control_to_dock_to.size.x, parent_top_left_position.y),
        "center_right" => Vec2::new(parent_top_left_position.x + control_to_dock_to.size.x, parent_top_left_position.y - control_to_dock_to.size.y * 0.5),
        "bottom_right" => Vec2::new(parent_top_left_position.x + control_to_dock_to.size.x, parent_top_left_position.y - control_to_dock_to.size.y),

        _ => panic!("{} is not implemented.", point_on_control_to_anchor_to)
    };
    //println!("Pixel1: {}", pixel1);

    let right_split_str = right.split(".").collect::<Vec<&str>>();
    let point_on_this_control_to_anchor_to = right_split_str[1];

    let pixel2 = match point_on_this_control_to_anchor_to {
        "top_left" => Vec2::new(pixel1.x, pixel1.y),
        "center_left" => Vec2::new(pixel1.x, pixel1.y + control.size.y * 0.5),
        "bottom_left" => Vec2::new(pixel1.x, pixel1.y + control.size.y),

        "top_middle" => Vec2::new(pixel1.x - control.size.x * 0.5, pixel1.y),
        "center_middle" => Vec2::new(pixel1.x - control.size.x * 0.5, pixel1.y + control.size.y * 0.5),
        "bottom_middle" => Vec2::new(pixel1.x - control.size.x * 0.5, pixel1.y + control.size.y),

        "top_right" => Vec2::new(pixel1.x - control.size.x, pixel1.y),
        "center_right" => Vec2::new(pixel1.x - control.size.x, pixel1.y + control.size.y * 0.5),
        "bottom_right" => Vec2::new(pixel1.x - control.size.x, pixel1.y + control.size.y),

        _ => panic!("{} is not implemented.", point_on_this_control_to_anchor_to)
    };
    //println!("Pixel2: {}", result_pixel);

    let result_pixel = pixel2 + control.offset;

    return result_pixel;
}

fn get_string(str: String) -> String {
    //let split_str = str.split(':').collect::<Vec<&str>>();
    //let right_side_of_colon = split_str[1].trim();
    let right_side_of_colon = get_right_side_of_colon(str);
    let result = right_side_of_colon.to_string();

    return result
}

fn get_f32(str: String) -> f32 {
    let right_side_of_colon = get_right_side_of_colon(str);
    let result = right_side_of_colon.parse::<f32>().unwrap();

    return result;
}

fn get_vec2(str: String) -> Vec2 {
    let right_side_of_colon = get_right_side_of_colon(str);
    let split_str3 = right_side_of_colon.split(';').collect::<Vec<&str>>();
    let value1 = split_str3[0].trim();
    let value2 = split_str3[1].trim();
    let result = Vec2::new(value1.parse::<f32>().unwrap(), value2.parse::<f32>().unwrap());

    return result;
}

fn get_color(str: String) -> Color {
    let right_side_of_colon = get_right_side_of_colon(str);
    let split_str3 = right_side_of_colon.split(';').collect::<Vec<&str>>();
    let value1 = split_str3[0].trim(); // red
    let value2 = split_str3[1].trim(); // green
    let value3 = split_str3[2].trim(); // blue
    let mut value4 = "1.0";
    if split_str3.len() == 4 {
        value4 = split_str3[3].trim();
    }
    
    let result = Color::Rgba { red: value1.parse::<f32>().unwrap(), green: value2.parse::<f32>().unwrap(), blue: value3.parse::<f32>().unwrap(), alpha: value4.parse::<f32>().unwrap() };

    return result;
}

fn get_right_side_of_colon(str: String) -> String {
    let split_str1 = str.split("//").collect::<Vec<&str>>();
    let split_str2 = split_str1[0].split(':').collect::<Vec<&str>>();
    let right_side_of_colon = split_str2[1].trim();

    return right_side_of_colon.to_string();
}

fn instantiate_sprite_bundle(
    size: Vec2,
    center_position: Vec3,
    scale: Vec3,
    texture_handle: Handle<ColorMaterial>
) -> SpriteBundle {

    let sprite = Sprite::new(size);
    let transform = Transform {
        translation: center_position,
        scale,
        ..Default::default()
    };

    let bundle = SpriteBundle {
        transform,
        sprite,
        material: texture_handle.clone(),
        ..Default::default()
    };

    return bundle;
}

fn instantiate_textbundle(
    top_left_position: Vec2,
    min_size: Vec2,
    max_size: Vec2,
    text: String,
    font_handle: Handle<Font>,
    font_size: f32,
    color: Color
) -> TextBundle {
    let position_type = PositionType::Absolute;
    let position = Rect {
        left: Val::Px(top_left_position.x),
        top: Val::Px(top_left_position.y),
        ..Default::default()
    };
    let style = Style {
        position_type,
        position,
        min_size: Size {
            width: Val::Px(min_size.x),
            height: Val::Px(min_size.y)
        },
        max_size: Size {
            width: Val::Px(max_size.x),
            height: Val::Px(max_size.y)
        },
        ..Default::default()
    };
    let text = Text::with_section(
        text,
        TextStyle { font: font_handle.clone(), font_size: font_size, color: color },
        TextAlignment {
            horizontal: HorizontalAlign::Left,
            vertical: VerticalAlign::Top,
        },
    );
    let bundle = TextBundle {
        style,
        text,
        ..Default::default()
    };

    return bundle;
}
