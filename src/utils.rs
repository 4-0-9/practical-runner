use sdl2::pixels::Color;

#[derive(Debug, Clone)]
pub struct ColorParseError;

pub fn color_from_hex(hex: &str) -> Result<Color, ColorParseError> {
    if hex.len() != 7 || hex.chars().nth(0).unwrap() != '#' {
        return Err(ColorParseError);
    }

    let r = u8::from_str_radix(&hex[1..3], 16).or(Err(ColorParseError))?;
    let g = u8::from_str_radix(&hex[3..5], 16).or(Err(ColorParseError))?;
    let b = u8::from_str_radix(&hex[5..7], 16).or(Err(ColorParseError))?;

    Ok(Color::RGB(r, g, b))
}
