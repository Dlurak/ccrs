use color_convert::color::Color;
use crate::utils::ColorFormat;

pub fn convert(input: &str, format: &ColorFormat) -> Result<String, ()> {
    let input = Color::new(input);
    let conv = match format {
        ColorFormat::Hex => input.to_hex(),
        ColorFormat::Rgb => input.to_rgb(),
        ColorFormat::Hsl => input.to_hsl(),
    };

    match conv {
        Ok(s) => Ok(s),
        Err(_) => Err(())
    }
}
