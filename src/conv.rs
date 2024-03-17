use crate::utils::ColorFormat;
use color_convert::color::Color;

pub fn convert(input: &str, format: &ColorFormat) -> Result<String, ()> {
    let input = Color::new(input);
    let conv = match format {
        ColorFormat::Hex => input.to_hex(),
        ColorFormat::Rgb => input.to_rgb(),
        ColorFormat::Hsl => input.to_hsl(),
    };

    match conv {
        Ok(s) => Ok(s),
        Err(_) => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_colors() {
        assert_eq!(convert(&"invalid", &ColorFormat::Hex), Err(()));
    }

    #[test]
    fn from_hex() {
        let hex = "#ff0000";
        assert_eq!(convert(&hex, &ColorFormat::Hex), Ok(hex.to_string()));
        assert_eq!(
            convert(&hex, &ColorFormat::Rgb),
            Ok(String::from("rgb(255,0,0)"))
        );
        assert_eq!(
            convert(&hex, &ColorFormat::Hsl),
            Ok(String::from("hsl(0,100%,50%)"))
        );
    }

    #[test]
    fn from_rgb() {
        let rgb = "rgb(255,0,0)";
        assert_eq!(
            convert(&rgb, &ColorFormat::Hex),
            Ok(String::from("#ff0000"))
        );
        assert_eq!(
            convert(&rgb, &ColorFormat::Hsl),
            Ok(String::from("hsl(0,100%,50%)"))
        );
    }

    #[test]
    fn from_hsl() {
        let hsl = "hsl(0,100%,50%)";
        assert_eq!(convert(&hsl, &ColorFormat::Hsl), Ok(hsl.to_string()));
        assert_eq!(
            convert(&hsl, &ColorFormat::Hex),
            Ok(String::from("#ff0000"))
        );
        assert_eq!(
            convert(&hsl, &ColorFormat::Rgb),
            Ok(String::from("rgb(255,0,0)"))
        );
    }
}
