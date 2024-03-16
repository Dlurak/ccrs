use clap::Parser;
use serde::Serialize;
use std::process;

mod conv;
mod utils;

use utils::ColorFormat;

#[derive(Parser, Serialize, Debug)]
#[clap(
    name = "ccrs",
    about = "Color Converter Rust can convert a color into other formats",
    after_help = "For more information and the source code, visit https://github.com/dlurak/ccrs.",
    setting(clap::AppSettings::ColoredHelp),
)]
struct Args {
    /// Specify the output color formats. Formats can be duplicates, no formats will result in no
    /// output
    #[clap(
        short,
        long,
        value_enum,
        default_values_t = [
            ColorFormat::Hex,
            ColorFormat::Rgb,
            ColorFormat::Hsl,
        ]
    )]
    out: Vec<ColorFormat>,

    /// The input color to convert
    color: String
}

fn main() {
    let cli = Args::parse();

    let colors = cli.out
        .iter()
        .map(|f| {
            let conv = conv::convert(&cli.color, &f);
            match conv {
                Ok(co) => (f, co),
                Err(_) => {
                    println!("Could not convert {} to {:?}", cli.color, f);
                    process::exit(1);
                }
            }
        })
        .collect::<Vec<_>>();

    for color in colors {
        println!("{:?} {}", color.0, color.1);
    }
}
