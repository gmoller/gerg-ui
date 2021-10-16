use bevy::prelude::Color;
use phf::phf_map;

pub fn parse_color(s: &String) -> Color {

    let split = s.trim().split(';').collect::<Vec<&str>>();
    if split.len() >= 3 {
        let value1 = split[0].trim(); // red
        let value2 = split[1].trim(); // green
        let value3 = split[2].trim(); // blue
        let mut value4 = "1.0";
        if split.len() == 4 {
            value4 = split[3].trim(); // alpha
        }
        
        let result = Color::Rgba { red: value1.parse::<f32>().unwrap(), green: value2.parse::<f32>().unwrap(), blue: value3.parse::<f32>().unwrap(), alpha: value4.parse::<f32>().unwrap() };
    
        return result;
    } else {
        let col = COLORS.get(split[0].to_lowercase().as_str()).cloned();

        match col {
            Some(c) => return c,
            None => panic!("Color [{}] unknown.", s)
        }
    }
}

static COLORS: phf::Map<&'static str, Color> = phf_map! {
    "air force blue" => Color::Rgba { red: 0.365, green: 0.541, blue: 0.659, alpha: 1.0 },
    "alice blue" => Color::Rgba { red: 0.941, green: 0.973, blue: 1.0, alpha: 1.0 },
    "alizarin crimson" => Color::Rgba { red: 0.89, green: 0.149, blue: 0.212, alpha: 1.0 },
    "almond" => Color::Rgba { red: 0.937, green: 0.871, blue: 0.804, alpha: 1.0 },
    "amaranth" => Color::Rgba { red: 0.898, green: 0.169, blue: 0.314, alpha: 1.0 },
    "amber" => Color::Rgba { red: 1.0, green: 0.749, blue: 0.0, alpha: 1.0 },
    "american rose" => Color::Rgba { red: 1.0, green: 0.012, blue: 0.243, alpha: 1.0 },
    "amethyst" => Color::Rgba { red: 0.6, green: 0.4, blue: 0.8, alpha: 1.0 },
    "android green" => Color::Rgba { red: 0.643, green: 0.776, blue: 0.224, alpha: 1.0 },
    "anti-flash white" => Color::Rgba { red: 0.949, green: 0.953, blue: 0.957, alpha: 1.0 },
    "antique brass" => Color::Rgba { red: 0.804, green: 0.584, blue: 0.459, alpha: 1.0 },
    "antique fuchsia" => Color::Rgba { red: 0.569, green: 0.361, blue: 0.514, alpha: 1.0 },
    "antique white" => Color::Rgba { red: 0.98, green: 0.922, blue: 0.843, alpha: 1.0 },
    "ao" => Color::Rgba { red: 0.0, green: 0.502, blue: 0.0, alpha: 1.0 },
    "apple green" => Color::Rgba { red: 0.553, green: 0.714, blue: 0.0, alpha: 1.0 },
    "apricot" => Color::Rgba { red: 0.984, green: 0.808, blue: 0.694, alpha: 1.0 },
    "aqua" => Color::Rgba { red: 0.0, green: 1.0, blue: 1.0, alpha: 1.0 },
    "aquamarine" => Color::Rgba { red: 0.498, green: 1.0, blue: 0.831, alpha: 1.0 },
    "army green" => Color::Rgba { red: 0.294, green: 0.325, blue: 0.125, alpha: 1.0 },
    "arylide yellow" => Color::Rgba { red: 0.914, green: 0.839, blue: 0.42, alpha: 1.0 },
    "ash grey" => Color::Rgba { red: 0.698, green: 0.745, blue: 0.71, alpha: 1.0 },
    "asparagus" => Color::Rgba { red: 0.529, green: 0.663, blue: 0.42, alpha: 1.0 },
    "atomic tangerine" => Color::Rgba { red: 1.0, green: 0.6, blue: 0.4, alpha: 1.0 },
    "auburn" => Color::Rgba { red: 0.647, green: 0.165, blue: 0.165, alpha: 1.0 },
    "aureolin" => Color::Rgba { red: 0.992, green: 0.933, blue: 0.0, alpha: 1.0 },
    "aurometalsaurus" => Color::Rgba { red: 0.431, green: 0.498, blue: 0.502, alpha: 1.0 },
    "awesome" => Color::Rgba { red: 1.0, green: 0.125, blue: 0.322, alpha: 1.0 },
    "azure" => Color::Rgba { red: 0.0, green: 0.498, blue: 1.0, alpha: 1.0 },
    "azure mist/web" => Color::Rgba { red: 0.941, green: 1.0, blue: 1.0, alpha: 1.0 },

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