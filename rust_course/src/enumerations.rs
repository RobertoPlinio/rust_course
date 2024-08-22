#![allow(dead_code)]
#![allow(unreachable_patterns)]

enum Color {
    Red,
    Green,
    Blue,
    Rgb(u8, u8, u8), // tuple
    Cmyk {
        // struct-like
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}

pub fn enumerations() {
    println!("\nenumerations! ---------------------------\n");

    let red_color = Color::Red;
    let yellow_color_rgb = Color::Rgb(1, 1, 0);
    let black_color_rgb = Color::Rgb(0, 0, 0);
    let magenta_color = Color::Cmyk {
        cyan: 0,
        magenta: 255,
        yellow: 0,
        black: 0,
    };
    let black_color_cmyk = Color::Cmyk {
        cyan: 0,
        magenta: 0,
        yellow: 0,
        black: 255,
    };

    print_color(red_color, "Red".to_string());
    print_color(yellow_color_rgb, "Yellow RGB".to_string());
    print_color(black_color_rgb, "Black RGB".to_string());
    print_color(magenta_color, "Magenta".to_string());
    print_color(black_color_cmyk, "Black CMYK".to_string());
}

fn print_color(color: Color, name: String) {
    print!("Color {} is ", name);

    match color {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),

        // there is an OR operator here
        Color::Rgb(0, 0, 0)
        | Color::Cmyk {
            black: 250..=255, // Range is cool
            .. // The two dots indicate that the other values can be discarded
        } => println!("black"),

        Color::Rgb(r, g, b) => println!("rgb({},{},{})", r, g, b),
        Color::Cmyk {
            cyan,
            magenta,
            yellow,
            black,
        } => println!("cmyk({},{},{},{})", cyan, magenta, yellow, black),
        _ => (), // if anything else, do nothing, ion this case this is
                 // an unreachable pattern
    }
}
