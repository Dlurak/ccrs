use clap::Parser;
use serde::Serialize;

#[derive(
    clap::ValueEnum, Clone, Default, Debug, Serialize,
)]
enum ColorFormat {
    #[default]
    Hex,
    Rgb,
    Hsl,
}

#[derive(Parser, Serialize, Debug)]
struct Args {
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
    color: String
}

fn main() {
    let cli = Args::parse();

    println!("{:?}", cli);
}
