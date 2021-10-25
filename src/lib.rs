use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use bevy::prelude::*;
use plugin::{ButtonState, GergButton, GergControl, GergLabel, GergPictureBox};

use crate::colors::parse_color;

mod colors;
pub mod plugin;

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
    fields: Fields
}

pub struct Controls {
    map: HashMap<String, Control>
}
impl Controls {
    fn get_by_name(&self, name: String) -> &Control {
        let item = self.map.get(&name);
        let result = match item {
            None => panic!("Control with name [{}] not found.", name),
            Some(control) => control
        };

        return result;
    }
}

#[derive(Default)]
struct Fields {
    map: HashMap<String, String>
}
impl Fields {
    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }
    fn get_by_name(&self, name: &str) -> &String {
        let item = self.map.get(name);
        let result = match item {
            None => panic!("Field [{}] not found in control fields.", name),
            Some(control) => control
        };

        return result;
    }
}

enum ControlType {
    PictureBox,
    Label,
    Button
}
impl Default for ControlType {
    fn default() -> Self { ControlType::PictureBox }
}

enum ReadState {
    None,
    GlobalSettings,
    Control
}

pub fn instantiate_controls_from_file(filename: &str) -> Controls {
    let lines = read_ui_file(filename);
    let controls = instantiate_controls(lines);

    return controls;
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
    control.fields.insert("dock_with".to_string(), "".to_string());
    control.fields.insert("offset".to_string(), "0;0".to_string());

    for line in lines {

        line_number += 1;
        if line.is_empty() { continue; } // skip line
        if line.trim().starts_with("//") { continue; } // skip line

        if line.trim().starts_with("--") {
            match line.to_lowercase().as_str() {
                "--global_settings--" => { },
                "--picture_box--" => { },
                "--label--" => { },
                "--button--" => { },
                "--end--" => { },
                _ => {
                    panic!("Control type [{}] unknown.", line);
                }
            }
        }

        match line.to_lowercase().as_str() {
            "--global_settings--" => {
                read_state = ReadState::GlobalSettings;
                global_settings = GlobalSettings { ..Default::default() };
                global_settings.font_size = "20".to_string();
                global_settings.color = "255;255;255".to_string();
            },
            "--picture_box--" => {
                read_state = ReadState::Control;
                control.control_type = ControlType::PictureBox;
                control.fields.insert("draw_order".to_string(), "0.0".to_string());
                control.fields.insert("top_left_position".to_string(), "".to_string());
                control.fields.insert("center_position".to_string(), "".to_string());
                control.fields.insert("color".to_string(), global_settings.color.clone());
            },
            "--label--" => {
                read_state = ReadState::Control;
                control.control_type = ControlType::Label;
                control.fields.insert("top_left_position".to_string(), "".to_string());
                control.fields.insert("center_position".to_string(), "".to_string());
                control.fields.insert("color".to_string(), global_settings.color.clone());
                control.fields.insert("font_name".to_string(), global_settings.font_name.clone());
                control.fields.insert("font_size".to_string(), global_settings.font_size.clone());
                control.fields.insert("text_string".to_string(), "".to_string());
            },
            "--button--" => {
                read_state = ReadState::Control;
                control.control_type = ControlType::Button;
                control.fields.insert("texture_name_hover".to_string(), "".to_string());
                control.fields.insert("texture_name_active".to_string(), "".to_string());
                control.fields.insert("texture_name_disabled".to_string(), "".to_string());
                control.fields.insert("on_click_sound".to_string(), "".to_string());
                control.fields.insert("draw_order".to_string(), "0.0".to_string());
                control.fields.insert("top_left_position".to_string(), "".to_string());
                control.fields.insert("center_position".to_string(), "".to_string());
                control.fields.insert("color".to_string(), global_settings.color.clone());
            }
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
                control.fields.insert("dock_with".to_string(), "".to_string());
                control.fields.insert("offset".to_string(), "0;0".to_string());
            },
            _ => {
                let split = line.split(':').collect::<Vec<&str>>();
                let field_name = split[0].to_lowercase();
                let field_value = get_string(line.clone());

                match read_state {
                    ReadState::None => { panic!("Not in a valid state. Line #{}: {}.", line_number, line); },
                    ReadState::GlobalSettings => {
                        match field_name.to_lowercase().as_str() {
                            "font_name" => { global_settings.font_name = field_value; },
                            "font_size" => { global_settings.font_size = field_value; },
                            "color" => { global_settings.color = field_value; },
                            _ => { panic!("Unknown field. Line#{}: {}.", line_number, line); }
                        }
                    },
                    ReadState::Control => {
                        if field_name == "name" {
                            control.name = field_value;
                        }

                        let field_value = get_string(line.clone());
                        control.fields.insert(field_name, field_value);
                    }
                }
            }
        }
    }

    let result = Controls { map: controls };

    return result;
}

pub fn spawn_controls(commands: &mut Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>, controls: Controls, screen_size: Vec2, control_group_name: String) -> Vec<Entity> {

    let mut results = Vec::new();
    let controls_map = &controls.map;
    for (_, control) in controls_map {

        let size = parse_vec2(control.fields.get_by_name("size"));
        let top_left_position = calculate_top_left_position(control, &controls, screen_size);

        match control.control_type {
            ControlType::PictureBox => {
                let entity = spawn_picture_box(top_left_position, size, control, &asset_server, &mut materials, commands, &control_group_name);

                results.push(entity);
            },
            ControlType::Label => {
                let entity = spawn_label(top_left_position, screen_size, control, &asset_server, size, commands, &control_group_name);

                results.push(entity);
            },
            ControlType::Button => {
                let entity = spawn_button(top_left_position, size, control, &asset_server, &mut materials, commands, &control_group_name);

                results.push(entity);
            }
        }

        //results.push(entity);
    }

    return results;
}

fn spawn_picture_box(top_left_position: Vec2, size: Vec2, control: &Control, asset_server: &Res<AssetServer>, materials: &mut ResMut<Assets<ColorMaterial>>, commands: &mut Commands, control_group_name: &String) -> Entity {
    let center_position = Vec3::new(top_left_position.x + size.x * 0.5, top_left_position.y - size.y * 0.5, parse_f32(control.fields.get_by_name("draw_order")));
    let scale = Vec3::new(1.0, 1.0, 1.0);
    let texture_name = control.fields.get_by_name("texture_name");
    let color_material_handle = get_color_material_handle(texture_name, asset_server, control, materials);
    let bundle = instantiate_sprite_bundle(size, center_position, scale, color_material_handle, true);
    let entity = commands
        .spawn_bundle(bundle)
        .insert(GergPictureBox { name: control.fields.get_by_name("name").clone() })
        .insert(GergControl { group_name: control_group_name.clone() })
        .id();

    println!("Spawned: {}, {}", control.fields.get_by_name("name").clone(), control_group_name.clone());

    return entity;
}

fn spawn_label(top_left_position: Vec2, screen_size: Vec2, control: &Control, asset_server: &Res<AssetServer>, size: Vec2, commands: &mut Commands, control_group_name: &String) -> Entity {
    let top_left_position = Vec2::new(top_left_position.x + screen_size.x * 0.5, screen_size.y * 0.5 - top_left_position.y);
    //top_left_position.x += screen_size.x * 0.5;
    //top_left_position.y = screen_size.y * 0.5 - top_left_position.y;
    let min_size = Vec2::new(0.0, 0.0);
    let text = control.fields.get_by_name("text_string");
    let font_handle: Handle<Font> = asset_server.load(format!("fonts/{}", control.fields.get_by_name("font_name")).as_str());
    let font_size = parse_f32(control.fields.get_by_name("font_size"));
    let color = to_bevy_color(parse_color(control.fields.get_by_name("color")));
    let bundle = instantiate_textbundle(top_left_position, min_size, size, text, font_handle, font_size, color);
    let entity = commands
        .spawn_bundle(bundle)
        .insert(GergLabel { name: control.fields.get_by_name("name").clone() })
        .insert(GergControl { group_name: control_group_name.clone() })
        .id();

    println!("Spawned: {}, {}", control.fields.get_by_name("name").clone(), control_group_name.clone());

    return entity;
}

fn spawn_button(top_left_position: Vec2, size: Vec2, control: &Control, asset_server: &Res<AssetServer>, materials: &mut ResMut<Assets<ColorMaterial>>, commands: &mut Commands, control_group_name: &String) -> Entity {
    let center_position = Vec3::new(top_left_position.x + size.x * 0.5, top_left_position.y - size.y * 0.5, parse_f32(control.fields.get_by_name("draw_order")));
    let scale = Vec3::new(1.0, 1.0, 1.0);
    let texture_name_normal = control.fields.get_by_name("texture_name_normal");
    let mut texture_name_hover = control.fields.get_by_name("texture_name_hover");
    if texture_name_hover.is_empty() {
        texture_name_hover = texture_name_normal;
    }
    let mut texture_name_active = control.fields.get_by_name("texture_name_active");
    if texture_name_active.is_empty() {
        texture_name_active = texture_name_normal;
    }
    let mut texture_name_disabled = control.fields.get_by_name("texture_name_disabled");
    if texture_name_disabled.is_empty() {
        texture_name_disabled = texture_name_normal;
    }
    let color_material_handle_normal = get_color_material_handle(texture_name_normal, asset_server, control, materials);
    let color_material_handle_hover = get_color_material_handle(texture_name_hover, asset_server, control, materials);
    let color_material_handle_active = get_color_material_handle(texture_name_active, asset_server, control, materials);
    let color_material_handle_disabled = get_color_material_handle(texture_name_disabled, asset_server, control, materials);
    let on_click_sound = control.fields.get_by_name("on_click_sound");
    let bundle = instantiate_sprite_bundle(size, center_position, scale, color_material_handle_normal.clone(), true);
    let entity = commands
        .spawn_bundle(bundle)
        .insert(GergButton {
            name: control.fields.get_by_name("name").clone(),
            button_state: ButtonState::Normal,
            color_material_handle_normal: color_material_handle_normal,
            color_material_handle_hover: color_material_handle_hover,
            color_material_handle_active: color_material_handle_active,
            color_material_handle_disabled: color_material_handle_disabled,
            on_click_sound: on_click_sound.to_string()
        })
        .insert(GergControl { group_name: control_group_name.clone() })
        .id();

    println!("Spawned: {}, {}", control.fields.get_by_name("name").clone(), control_group_name.clone());

    return entity;
}

fn get_color_material_handle(path: &str, asset_server: &Res<AssetServer>, control: &Control, materials: &mut ResMut<Assets<ColorMaterial>>) -> Handle<ColorMaterial> {
    let mut color_material: ColorMaterial = asset_server.load(path).into();
    let color = control.fields.get_by_name("color");
    if !color.is_empty() {
        let color = to_bevy_color(parse_color(color));
        color_material.color = color;
    }
    let color_material_handle = materials.add(color_material);
    
    return color_material_handle;
}

fn to_bevy_color(color_u32: u32) -> Color {
    let color_bytes= color_u32.to_le_bytes();
    let r = color_bytes[0] as f32 / 255.0;
    let g = color_bytes[1] as f32 / 255.0;
    let b = color_bytes[2] as f32 / 255.0;
    let a = color_bytes[3] as f32 / 255.0;

    let color = Color::Rgba { red: r, green: g, blue: b, alpha: a};

    return color;
}

fn calculate_top_left_position(control: &Control, controls: &Controls, screen_size: Vec2) -> Vec2 {

    let control_size = parse_vec2(control.fields.get_by_name("size"));
    
    let control_dock_with = control.fields.get_by_name("dock_with");
    if control_dock_with.is_empty() {
        let top_left_position = control.fields.get_by_name("top_left_position");

        if top_left_position.is_empty() {
            let center_position = control.fields.get_by_name("center_position");

            if center_position.is_empty() {
                return Vec2::new(0.0 - control_size.x * 0.5, 0.0 + control_size.y * 0.5);
            } else {
                let cp = parse_vec2(&center_position);
                let tlp = Vec2::new(cp.x - control_size.x * 0.5, cp.y + control_size.y * 0.5);
                
                return tlp;
            }
        } else {
            return parse_vec2(&top_left_position);
        }
    }

    let split = control_dock_with.split("<->").collect::<Vec<&str>>();
    let dock_this = split[1];
    let dock_to = split[0];

    let split = dock_to.split(".").collect::<Vec<&str>>();
    let control_to_use_for_docking = split[0];
    let point_on_control_to_anchor_to = split[1];

    let pixel1 = get_point_to_dock_to(controls, control_to_use_for_docking, point_on_control_to_anchor_to, screen_size);
    //println!("Pixel1: {}", pixel1);

    let split = dock_this.split(".").collect::<Vec<&str>>();
    let point_on_this_control_to_anchor_to = split[1];

    let pixel2 = match point_on_this_control_to_anchor_to.to_lowercase().as_str() {
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

    let offset = control.fields.get_by_name("offset");
    let result_pixel = pixel2 + parse_vec2(offset);

    return result_pixel;
}

fn get_point_to_dock_to(controls: &Controls, control_to_use_for_docking: &str, point_on_control_to_anchor_to: &str, screen_size: Vec2) -> Vec2 {

    if control_to_use_for_docking.to_lowercase() == "screen" {
        let p = match point_on_control_to_anchor_to.to_lowercase().as_str() {
            "top_left" => Vec2::new(-screen_size.x * 0.5, screen_size.y * 0.5),
            "center_left" => Vec2::new(-screen_size.x * 0.5, 0.0),
            "bottom_left" => Vec2::new(-screen_size.x * 0.5, -screen_size.y * 0.5),
        
            "top_middle" => Vec2::new(0.0, screen_size.y * 0.5),
            "center_middle" => Vec2::new(0.0, 0.0),
            "bottom_middle" => Vec2::new(0.0, -screen_size.y * 0.5),
    
            "top_right" => Vec2::new(screen_size.x * 0.5, screen_size.y * 0.5),
            "center_right" => Vec2::new(screen_size.x * 0.5, 0.0),
            "bottom_right" => Vec2::new(screen_size.x * 0.5, -screen_size.y * 0.5),
    
            _ => panic!("{} is not implemented.", point_on_control_to_anchor_to)
        };
    
        return p;
    }

    let control_to_dock_to = controls.get_by_name(control_to_use_for_docking.to_string());
    let control_to_dock_to_size = parse_vec2(control_to_dock_to.fields.get_by_name("size"));
    let parent_top_left_position = calculate_top_left_position(control_to_dock_to, controls, screen_size);
    let p = match point_on_control_to_anchor_to.to_lowercase().as_str() {
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

    return p;
}

fn get_string(s: String) -> String {

    let right_side_of_colon = get_right_side_of_colon(s);
    let result = right_side_of_colon.to_string();

    return result;
}

fn parse_f32(s: &String) -> f32 {

    let result = s.parse::<f32>().unwrap();

    return result;
}

fn parse_vec2(s: &String) -> Vec2 {
    
    if s.is_empty() {
        return Vec2::new(0.0, 0.0);
    }

    let split = s.split(';').collect::<Vec<&str>>();
    let value1 = split[0].trim();
    let value2 = split[1].trim();
    let result = Vec2::new(value1.parse::<f32>().unwrap(), value2.parse::<f32>().unwrap());

    return result;
}

fn get_right_side_of_colon(s: String) -> String {
    
    let split = s.split("//").collect::<Vec<&str>>();
    let split = split[0].split(':').collect::<Vec<&str>>();
    let right_side_of_colon = split[1].trim();

    return right_side_of_colon.to_string();
}

fn instantiate_sprite_bundle(
    size: Vec2,
    center_position: Vec3,
    scale: Vec3,
    material_handle: Handle<ColorMaterial>,
    is_visible: bool,
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
        visible: Visible { is_visible, is_transparent: true},
        material: material_handle.clone(),
        ..Default::default()
    };

    return bundle;
}

fn instantiate_textbundle(
    top_left_position: Vec2,
    min_size: Vec2,
    max_size: Vec2,
    text: &String,
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
