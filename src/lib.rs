use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use phf::phf_map;
use bevy::prelude::*;

#[derive(Default)]
struct GlobalSettings {
    font_name: String,
    font_size: String,
    color: String
}

#[derive(Default)]
struct Control {
    control_type: ControlType,
    name: String,
    center_position: String,
    top_left_position: String,
    size: String,
    dock_with: String,
    offset: String,

    // picture_box
    texture_name: String,
    draw_order: String,

    // label
    text_string: String,
    font_name: String,
    font_size: String,
    color: String,
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
    PictureBox,
    Label
}
impl Default for ControlType {
    fn default() -> Self { ControlType::PictureBox }
}

enum ReadState {
    None,
    GlobalSettings,
    Control
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
        if line.trim().starts_with("//") { continue; } // skip line

        match line.as_str() {
            "--global_settings--" => {
                read_state = ReadState::GlobalSettings;
                global_settings = GlobalSettings { ..Default::default() };
            },
            "--picture_box--" => {
                read_state = ReadState::Control;
                control.control_type = ControlType::PictureBox;

            },
            "--label--" => {
                read_state = ReadState::Control;
                control.control_type = ControlType::Label;
                control.font_name = global_settings.font_name.clone();
                control.font_size = global_settings.font_size.clone();
                control.color = global_settings.color.clone();
            },
            "--end--" => {
                match read_state {
                    ReadState::None => { panic!("End found while not in a valid state. Line #{}: {}.", line_number, line); },
                    ReadState::GlobalSettings => { }, // do nothing
                    ReadState::Control => {
                        controls.insert(control.name.clone(), control);
                    }
                }
                read_state = ReadState::None;
                control = Control { ..Default::default() };
            },
            _ => {
                let split_str = line.split(':').collect::<Vec<&str>>();
                let field_name = split_str[0];
                let field_value = get_string(line.clone());

                match read_state {
                    ReadState::None => { panic!("Not in a valid state. Line #{}: {}.", line_number, line); },
                    ReadState::GlobalSettings => {
                        match field_name {
                            "font_name" => { global_settings.font_name = field_value; },
                            "font_size" => { global_settings.font_size = field_value; },
                            "color" => { global_settings.color = field_value; },
                            _ => { panic!("Unknown field. Line#{}: {}.", line_number, line); }
                        }
                    },
                    ReadState::Control => {
                        match field_name {
                            "name" => { control.name = field_value; },
                            "top_left_position" => { control.top_left_position = field_value; },
                            "center_position" => { control.center_position = field_value; },
                            "size" => { control.size = field_value; },
                            "dock_with" => { control.dock_with = field_value; },
                            "offset" => { control.offset = field_value; },

                            "texture_name" => { control.texture_name = field_value; },
                            "draw_order" => { control.draw_order = field_value; },

                            "text_string" => { control.text_string = field_value; },
                            "font_name" => { control.font_name = field_value; },
                            "font_size" => { control.font_size = field_value; },
                            "color" => { control.color = field_value; },

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

pub fn spawn_controls(commands: &mut Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>, controls: Controls, screen_size: Vec2) -> Vec<Entity> {

    let mut results = Vec::new();
    let controls_map = &controls.map;
    for (_, control) in controls_map {

        match control.control_type {
            ControlType::PictureBox => {
                let scale = Vec3::new(1.0, 1.0, 1.0);
                let material_path = control.texture_name.as_str();
                let texture_handle = asset_server.load(material_path);
                let material = materials.add(texture_handle.into());
            
                let top_left_position = calculate_top_left_position(control, &controls);
                let size = parse_vec2(control.size.clone());
                let center_position = Vec3::new(top_left_position.x + size.x * 0.5, top_left_position.y - size.y * 0.5, parse_f32(control.draw_order.clone()));

                let bundle = instantiate_sprite_bundle(size, center_position, scale, material);
                let entity = commands.spawn_bundle(bundle).id();

                results.push(entity);
            },
            ControlType::Label => {
                let min_size = Vec2::new(0.0, 0.0);
                let font_path = format!("fonts/{}", control.font_name);
                let font_handle: Handle<Font> = asset_server.load(font_path.as_str());
            
                let mut top_left_position = calculate_top_left_position(control, &controls);
                top_left_position.x += screen_size.x * 0.5;
                top_left_position.y = screen_size.y * 0.5 - top_left_position.y;
                let size = parse_vec2(control.size.clone());

                let bundle = instantiate_textbundle(top_left_position, min_size, size, control.text_string.clone(), font_handle, parse_f32(control.font_size.clone()), parse_color(control.color.clone()));
                let entity = commands.spawn_bundle(bundle).id();

                results.push(entity);
            }
        }

        //results.push(entity);
    }

    return results;
}

fn calculate_top_left_position(control: &Control, controls: &Controls) -> Vec2 {

    let  control_size = parse_vec2(control.size.clone());
    
    if control.dock_with.is_empty() {
        let top_left_position = control.top_left_position.clone();

        if top_left_position.is_empty() {
            let center_position = control.center_position.clone();

            if center_position.is_empty() {
                return Vec2::new(0.0, 0.0);
            } else {
                let cp = parse_vec2(center_position);
                let tlp = Vec2::new(cp.x - control_size.x * 0.5, cp.y + control_size.y * 0.5);
                
                return tlp;
            }
        } else {
            return parse_vec2(top_left_position);
        }
    }

    let split_str = control.dock_with.split("<->").collect::<Vec<&str>>();
    let left = split_str[0];
    let right = split_str[1];

    let left_split_str = left.split(".").collect::<Vec<&str>>();
    let control_to_use_for_docking = left_split_str[0];
    let point_on_control_to_anchor_to = left_split_str[1];
    let control_to_dock_to = controls.get_by_name(control_to_use_for_docking.to_string());
    let  control_to_dock_to_size = parse_vec2(control_to_dock_to.size.clone());

    let parent_top_left_position = calculate_top_left_position(control_to_dock_to, controls);

    let pixel1 = match point_on_control_to_anchor_to {
        "top_left" => Vec2::new(parent_top_left_position.x, parent_top_left_position.y),
        "center_left" => Vec2::new(parent_top_left_position.x, parent_top_left_position.y - control_to_dock_to_size.y * 0.5),
        "bottom_left" => Vec2::new(parent_top_left_position.x, parent_top_left_position.y - control_to_dock_to_size.y),
        
        "top_middle" => Vec2::new(parent_top_left_position.x + control_to_dock_to_size.x * 0.5, parent_top_left_position.y),
        "center_middle" => Vec2::new(parent_top_left_position.x + control_to_dock_to_size.x * 0.5, parent_top_left_position.y - control_to_dock_to_size.y * 0.5),
        "bottom_middle" => Vec2::new(parent_top_left_position.x + control_to_dock_to_size.x * 0.5, parent_top_left_position.y - control_to_dock_to_size.y),

        "top_right" => Vec2::new(parent_top_left_position.x + control_to_dock_to_size.x, parent_top_left_position.y),
        "center_right" => Vec2::new(parent_top_left_position.x + control_to_dock_to_size.x, parent_top_left_position.y - control_to_dock_to_size.y * 0.5),
        "bottom_right" => Vec2::new(parent_top_left_position.x + control_to_dock_to_size.x, parent_top_left_position.y - control_to_dock_to_size.y),

        _ => panic!("{} is not implemented.", point_on_control_to_anchor_to)
    };
    //println!("Pixel1: {}", pixel1);

    let right_split_str = right.split(".").collect::<Vec<&str>>();
    let point_on_this_control_to_anchor_to = right_split_str[1];

    let pixel2 = match point_on_this_control_to_anchor_to {
        "top_left" => Vec2::new(pixel1.x, pixel1.y),
        "center_left" => Vec2::new(pixel1.x, pixel1.y + control_size.y * 0.5),
        "bottom_left" => Vec2::new(pixel1.x, pixel1.y + control_size.y),

        "top_middle" => Vec2::new(pixel1.x - control_size.x * 0.5, pixel1.y),
        "center_middle" => Vec2::new(pixel1.x - control_size.x * 0.5, pixel1.y + control_size.y * 0.5),
        "bottom_middle" => Vec2::new(pixel1.x - control_size.x * 0.5, pixel1.y + control_size.y),

        "top_right" => Vec2::new(pixel1.x - control_size.x, pixel1.y),
        "center_right" => Vec2::new(pixel1.x - control_size.x, pixel1.y + control_size.y * 0.5),
        "bottom_right" => Vec2::new(pixel1.x - control_size.x, pixel1.y + control_size.y),

        _ => panic!("{} is not implemented.", point_on_this_control_to_anchor_to)
    };
    //println!("Pixel2: {}", result_pixel);

    let result_pixel = pixel2 + parse_vec2(control.offset.clone());

    return result_pixel;
}

fn get_string(str: String) -> String {

    let right_side_of_colon = get_right_side_of_colon(str);
    let result = right_side_of_colon.to_string();

    return result
}

fn parse_f32(str: String) -> f32 {

    let result = str.parse::<f32>().unwrap();

    return result;
}

fn parse_vec2(str: String) -> Vec2 {
    
    if str.is_empty() {
        return Vec2::new(0.0, 0.0);
    }

    let split_str3 = str.split(';').collect::<Vec<&str>>();
    let value1 = split_str3[0].trim();
    let value2 = split_str3[1].trim();
    let result = Vec2::new(value1.parse::<f32>().unwrap(), value2.parse::<f32>().unwrap());

    return result;
}

static COLORS: phf::Map<&'static str, Color> = phf_map! {
    "american rose" => Color::Rgba { red: 1.0, green: 0.012, blue: 0.243, alpha: 1.0 },
    "apricot" => Color::Rgba { red: 0.984, green: 0.808, blue: 0.694, alpha: 1.0 },
    "aqua" => Color::Rgba { red: 0.0, green: 1.0, blue: 1.0, alpha: 1.0 },
    "black" => Color::Rgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 },
    "blue" => Color::Rgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 },
    "canary yellow" => Color::Rgba { red: 1.0, green: 0.937, blue: 0.0, alpha: 1.0 },
    "chocolate" => Color::Rgba { red: 0.824, green: 0.412, blue: 0.118, alpha: 1.0 },
    "cornflower blue" => Color::Rgba { red: 0.392, green: 0.584, blue: 0.929, alpha: 1.0 },
    "cyan" => Color::Rgba { red: 0.0, green: 1.0, blue: 1.0, alpha: 1.0 },
    "fuchsia" => Color::Rgba { red: 1.0, green: 0.0, blue: 1.0, alpha: 1.0 },
    "gray" => Color::Rgba { red: 0.502, green: 0.502, blue: 0.502, alpha: 1.0 },
    "green" => Color::Rgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 },
    "magenta" => Color::Rgba { red: 1.0, green: 0.0, blue: 1.0, alpha: 1.0 },
    "maroon" => Color::Rgba { red: 0.502, green: 0.0, blue: 0.0, alpha: 1.0 },
    "navy blue" => Color::Rgba { red: 0.0, green: 0.0, blue: 0.502, alpha: 1.0 },
    "olive" => Color::Rgba { red: 0.502, green: 0.502, blue: 0.0, alpha: 1.0 },
    "purple" => Color::Rgba { red: 0.502, green: 0.0, blue: 0.502, alpha: 1.0 },
    "red" => Color::Rgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 },
    "silver" => Color::Rgba { red: 0.753, green: 0.753, blue: 0.753, alpha: 1.0 },
    "teal" => Color::Rgba { red: 0.0, green: 0.502, blue: 0.502, alpha: 1.0 },
    "white" => Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 },
    "yellow" => Color::Rgba { red: 1.0, green: 1.0, blue: 0.0, alpha: 1.0 },
};

fn parse_color(str: String) -> Color {

    let trimmed_string = str.trim();

    let split_str3 = trimmed_string.split(';').collect::<Vec<&str>>();
    if split_str3.len() >= 3 {
        let value1 = split_str3[0].trim(); // red
        let value2 = split_str3[1].trim(); // green
        let value3 = split_str3[2].trim(); // blue
        let mut value4 = "1.0";
        if split_str3.len() == 4 {
            value4 = split_str3[3].trim(); // alpha
        }
        
        let result = Color::Rgba { red: value1.parse::<f32>().unwrap(), green: value2.parse::<f32>().unwrap(), blue: value3.parse::<f32>().unwrap(), alpha: value4.parse::<f32>().unwrap() };
    
        return result;
    } else {
        let col = COLORS.get(split_str3[0].to_lowercase().as_str()).cloned();

        match col {
            Some(c) => return c,
            None => panic!("Color [{}] unknown.", str)
        }
    }
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
