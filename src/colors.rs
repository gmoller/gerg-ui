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

    result
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

    result
}

fn from_rgb_decimal(s: &String) -> u32 {
    let split = s.trim().split(';').collect::<Vec<&str>>();
    if split.len() == 3 {
        let value1 = split[0].trim().parse::<u32>().unwrap(); // red
        let value2 = split[1].trim().parse::<u32>().unwrap(); // green
        let value3 = split[2].trim().parse::<u32>().unwrap(); // blue
        let value4: u32 = 255;
        
        let result = (value1 << 0) + (value2 << 8) + (value3 << 16) + (value4 << 24);
    
        return result;
    } else if split.len() == 4 {
        let value1 = split[0].trim().parse::<u32>().unwrap(); // red
        let value2 = split[1].trim().parse::<u32>().unwrap(); // green
        let value3 = split[2].trim().parse::<u32>().unwrap(); // blue
        let value4 = split[3].trim().parse::<u32>().unwrap(); // alpha

        let result = (value1 << 0) + (value2 << 8) + (value3 << 16) + (value4 << 24);
    
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
    "ash gray" => "#b2beb5",
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
    "battleship gray" => "#848482",
    "bazaar" => "#98777b",
    "beau blue" => "#bcd4e6",
    "beaver" => "#9f8170",
    "beige" => "#f5f5dc",
    "bisque" => "#ffe4c4",
    "bistre" => "#3d2b1f",
    "bittersweet" => "#fe6f5e",
    "black" => "#000000",
    "blanched almond" => "#ffebcd",
    "bleu de france" => "#318ce7",
    "blizzard blue" => "#ace5ee",
    "blond" => "#faf0be",
    "blue" => "#0000ff",
    "blue bell" => "#a2a2d0",
    "blue gray" => "#6699cc",
    "blue green" => "#0d98ba",
    "blue purple" => "#8a2be2",
    "blue violet" => "#8a2be2",
    "blush" => "#de5d83",
    "bole" => "#79443b",
    "bondi blue" => "#0095b6",
    "bone" => "#e3dac9",
    "boston university red" => "#cc0000",
    "bottle green" => "#006a4e",
    "boysenberry" => "#873260",
    "brandeis blue" => "#0070ff",
    "brass" => "#b5a642",
    "brick red" => "#cb4154",
    "bright cerulean" => "#1dacd6",
    "bright green" => "#66ff00",
    "bright lavender" => "#bf94e4",
    "bright maroon" => "#c32148",
    "bright pink" => "#ff007f",
    "bright turquoise" => "#08e8de",
    "bright ube" => "#d19fe8",
    "brilliant lavender" => "#f4bbff",
    "brilliant rose" => "#ff55a3",
    "brink pink" => "#fb607f",
    "british racing green" => "#004225",
    "bronze" => "#cd7f32",
    "brown" => "#a52a2a",
    "bubble gum" => "#ffc1cc",
    "bubbles" => "#e7feff",
    "buff" => "#f0dc82",
    "bulgarian rose" => "#480607",
    "burgundy" => "#800020",
    "burlywood" => "#deb887",
    "burnt orange" => "#cc5500",
    "burnt sienna" => "#e97451",
    "burnt umber" => "#8a3324",
    "byzantine" => "#bd33a4",
    "byzantium" => "#702963",

    "cg blue" => "#007aa5",
    "cg red" => "#e03c31",
    "cadet" => "#536872",
    "cadet blue" => "#5f9ea0",
    "cadet gray" => "#91a3b0",
    "cadmium green" => "#006b3c",
    "cadmium orange" => "#ed872d",
    "cadmium red" => "#e30022",
    "cadmium yellow" => "#fff600",
    "cafe au lait" => "#a67b5b",
    "cafe noir" => "#4b3621",
    "cal poly pomona green" => "#1e4d2b",
    "cambridge blue" => "#a3c1ad",
    "camel" => "#c19a6b",
    "camouflage green" => "#78866b",
    "canary" => "#ffff99",
    "canary yellow" => "#ffef00",
    "candy apple red" => "#ff0800",
    "candy pink" => "#e4717a",
    "capri" => "#00bfff",
    "caput mortuum" => "#592720",
    "cardinal" => "#c41e3a",
    "caribbean green" => "#00cc99",
    "carmine" => "#ff0040",
    "carmine pink" => "#eb4c42",
    "carmine red" => "#ff0038",
    "carnation pink" => "#ffa6c9",
    "carnelian" => "#b31b1b",
    "carolina blue" => "#99badd",
    "carrot orange" => "#ed9121",
    "celadon" => "#ace1af",
    "celeste" => "#b2ffff",
    "celestial blue" => "#4997d0",
    "cerise" => "#de3163",
    "cerise pink" => "#ec3b83",
    "cerulean" => "#007ba7",
    "cerulean blue" => "#2a52be",
    "chamoisee" => "#a0785a",
    "champagne" => "#fad6a5",
    "charcoal" => "#36454f",
    "chartreuse" => "#7fff00",
    "cherry" => "#de3163",
    "cherry blossom pink" => "#ffb7c5",
    "chestnut" => "#cd5c5c",
    "chocolate" => "#d2691e",
    "chrome yellow" => "#ffa700",
    "cinereous" => "#98817b",
    "cinnabar" => "#e34234",
    "cinnamon" => "#d2691e",
    "citrine" => "#e4d00a",
    "classic rose" => "#fbcce7",
    "cobalt" => "#0047ab",
    "cocoa brown" => "#d2691e",
    "coffee" => "#6f4e37",
    "columbia blue" => "#9bddff",
    "cool black" => "#002e63",
    "cool gray" => "#8c92ac",
    "copper" => "#b87333",
    "copper rose" => "#996666",
    "coquelicot" => "#ff3800",
    "coral" => "#ff7f50",
    "coral pink" => "#f88379",
    "coral red" => "#ff4040",
    "cordovan" => "#893f45",
    "corn" => "#fbec5d",
    "cornell red" => "#b31b1b",
    "cornflower" => "#9aceeb",
    "cornflower blue" => "#6495ed",
    "cornsilk" => "#fff8dc",
    "cosmic latte" => "#fff8e7",
    "cotton candy" => "#ffbcd9",
    "cream" => "#fffdd0",
    "crimson" => "#dc143c",
    "crimson red" => "#990000",
    "crimson glory" => "#be0032",
    "cyan" => "#00ffff",

    "daffodil" => "#ffff31",
    "dandelion" => "#f0e130",
    "dark blue" => "#00008b",
    "dark brown" => "#654321",
    "dark byzantium" => "#5d3954",
    "dark candy apple red" => "#a40000",
    "dark cerulean" => "#08457e",
    "dark chestnut" => "#986960",
    "dark coral" => "#cd5b45",
    "dark cyan" => "#008b8b",
    "dark electric blue" => "#536878",
    "dark goldenrod" => "#b8860b",
    "dark gray" => "#a9a9a9",
    "dark green" => "#013220",
    "dark jungle green" => "#1a2421",
    "dark khaki" => "#bdb76b",
    "dark lava" => "#483c32",
    "dark lavender" => "#734f96",
    "dark magenta" => "#8b008b",
    "dark midnight blue" => "#003366",
    "dark olive green" => "#556b2f",
    "dark orange" => "#ff8c00",
    "dark orchid" => "#9932cc",
    "dark pastel blue" => "#779ecb",
    "dark pastel green" => "#03c03c",
    "dark pastel purple" => "#966fd6",
    "dark pastel red" => "#c23b22",
    "dark pink" => "#e75480",
    "dark powder blue" => "#003399",
    "dark raspberry" => "#872657",
    "dark red" => "#8b0000",
    "dark salmon" => "#e9967a",
    "dark scarlet" => "#560319",
    "dark sea green" => "#8fbc8f",
    "dark sienna" => "#3c1414",
    "dark slate blue" => "#483d8b",
    "dark slate gray" => "#2f4f4f",
    "dark spring green" => "#177245",
    "dark tan" => "#918151",
    "dark tangerine" => "#ffa812",
    "dark taupe" => "#483c32",
    "dark terra cotta" => "#cc4e5c",
    "dark turquoise" => "#00ced1",
    "dark violet" => "#9400d3",
    "dartmouth green" => "#00693e",
    "davy gray" => "#555555",

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
