use hex::FromHex;
use phf::phf_map;

enum ConversionType {
    FromHexTriplet,
    FromRgbDecimal,
    //FromRgbPercent,
    FromName,
}

pub fn parse_color(s: &String) -> u32 {
    let conversion_type = determine_conversion_type(&s);

    let result = match conversion_type {
        ConversionType::FromHexTriplet => from_hex_triplet(s),
        ConversionType::FromRgbDecimal => from_rgb_decimal(s),
        ConversionType::FromName => from_name(s),
    };

    return result;
}

fn from_hex_triplet(s: &String) -> u32 {
    let mut s2 = s.clone();
    s2.remove(0);
    let buffer = <[u8;3]>::from_hex(s2);
    let result = match buffer {
        Ok(bytes) => {
            ((bytes[0] as u32) << 0) + ((bytes[1] as u32) << 8) + ((bytes[2] as u32) << 16) + (255 << 24)
        },
        Err(_) => panic!("[{}] is not a hex value.", s),
    };
    println!("from_hex_triplet: {}", result);

    return result;
}

fn from_rgb_decimal(s: &String) -> u32 {
    let split = s.trim().split(';').collect::<Vec<&str>>();
    if split.len() == 3 {
        let value1 = split[0].trim().parse::<u32>().unwrap(); // red
        let value2 = split[1].trim().parse::<u32>().unwrap(); // green
        let value3 = split[2].trim().parse::<u32>().unwrap(); // blue
        let value4: u32 = 255;
        
        let result = (value1 << 0) + (value2 << 8) + (value3 << 16) + (value4 << 24);
        println!("from_decimal: {}", result);
    
        return result;
    } else if split.len() == 4 {
        let value1 = split[0].trim().parse::<u32>().unwrap(); // red
        let value2 = split[1].trim().parse::<u32>().unwrap(); // green
        let value3 = split[2].trim().parse::<u32>().unwrap(); // blue
        let value4 = split[3].trim().parse::<u32>().unwrap(); // alpha

        let result = (value1 << 0) + (value2 << 8) + (value3 << 16) + (value4 << 24);
        println!("from_decimal: {}", result);
    
        return result;
    } else {
        panic!("Could not parse [{}]", s);
    }
}

fn from_name(s: &String) -> u32 {
    let col = COLORS.get(s.to_lowercase().as_str()).cloned();

    match col {
        Some(c) => return from_hex_triplet(&c.to_string()),
        None => panic!("Color [{}] unknown.", s)
    }
}

fn determine_conversion_type(s: &String) -> ConversionType {
    let s = s.trim();

    if s.starts_with('#') {
        return ConversionType::FromHexTriplet;
    }

    let s = s.split(';').collect::<Vec<&str>>();
    if s.len() >= 3 {
        return ConversionType::FromRgbDecimal;
    }

    return ConversionType::FromName;
}

// pub fn parse_color(s: &String) -> Color {

//     let split = s.trim().split(';').collect::<Vec<&str>>();
//     if split.len() >= 3 {
//         let value1 = split[0].trim(); // red
//         let value2 = split[1].trim(); // green
//         let value3 = split[2].trim(); // blue
//         let mut value4 = "1.0";
//         if split.len() == 4 {
//             value4 = split[3].trim(); // alpha
//         }
        
//         let result = Color::Rgba { red: value1.parse::<f32>().unwrap(), green: value2.parse::<f32>().unwrap(), blue: value3.parse::<f32>().unwrap(), alpha: value4.parse::<f32>().unwrap() };
    
//         return result;
//     } else {
//         let col = COLORS.get(split[0].to_lowercase().as_str()).cloned();

//         match col {
//             Some(c) => return c,
//             None => panic!("Color [{}] unknown.", s)
//         }
//     }
// }

static COLORS: phf::Map<&'static str, &str> = phf_map! {
    "air force blue" => "#5d8aa8",
    "alice blue" => "#f0f8ff",
    "alizarin crimson" => "#e32636",
    "almond" => "#efdecd",
    "amaranth" => "#e52b50",
    "amber" => "#ffbf00",
    "american rose" => "#ff033e",
    "amethyst" => "#9966cc",
    "android green" => "#a4c639",
    "anti-flash white" => "#f2f3f4",
    "antique brass" => "#cd9575",
    "antique fuchsia" => "#915c83",
    "antique white" => "#faebd7",
    "ao" => "#008000",
    "apple green" => "#8db600",
    "apricot" => "#fbceb1",
    "aqua" => "#00ffff",
    "aquamarine" => "#7fffd4",
    "army green" => "#4b5320",
    "arylide yellow" => "#e9d66b",
    "ash grey" => "#b2beb5",
    "asparagus" => "#87a96b",
    "atomic tangerine" => "#ff9966",
    "auburn" => "#a52a2a",
    "aureolin" => "#fdee00",
    "aurometalsaurus" => "#6e7f80",
    "awesome" => "#ff2052",
    "azure" => "#007fff",
    "azure mist/web" => "#f0ffff",

    "baby blue" => "#89cff0",
    "baby blue eyes" => "#a1caf1",
    "baby pink" => "#f4c2c2",
    "ball blue" => "#21abcd",
    "banana mania" => "#fae7b5",
    "banana yellow" => "#ffe135",
    "battleship grey" => "#848482",
    "bazaar" => "#98777b",
    "beau blue" => "#bcd4e6",
    "beaver" => "#9f8170",
    "beige" => "#f5f5dc",
    "bisque" => "#ffe4c4",
    "bistre" => "#3d2b1f",
    "bittersweet" => "#fe6f5e",
    "black" => "#000000",
    "blue" => "#0000ff",

    "canary yellow" => "#ffef00",
    "chocolate" => "#d2691e",
    "cornflower blue" => "#6495ed",
    "cyan" => "#00ffff",

    "fuchsia" => "#ff00ff",

    "gray" => "#808080",
    "green" => "#00ff00",

    "magenta" => "#ff00ff",
    "maroon" => "#800000",

    "navy blue" => "#000080",

    "olive" => "#808000",

    "purple" => "#800080",

    "red" => "#ff0000",

    "silver" => "#c0c0c0",

    "teal" => "#008080",

    "white" => "#ffffff",

    "yellow" => "#ffff00",
};