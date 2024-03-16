use serde::Serialize;

#[derive(
    clap::ValueEnum, Clone, Default, Debug, Serialize,
)]
pub enum ColorFormat {
    #[default]
    Hex,
    Rgb,
    Hsl,
}
