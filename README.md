# CCRS

## Color converter rust

Color Converter Rust can convert a color into other formats.

Supported formats:

- Hex
- Hsl
- Rgb

## Usage

```bash
$ ccrs [OPTIONS] <COLOR>
```

### Arguments

| Argument | Description                |
| -------- | -------------------------- |
| COLOR    | The input color to convert |

### Options

| Short | Long     | Description                                                                                                                                              |
| ----- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-h`  | `--help` | Print help information                                                                                                                                   |
| `-o`  | `--out`  | Specify the output color formats. Formats can be duplicates, no formats will result in no output [default: hex rgb hsl] [possible values: hex, rgb, hsl] |

## Installation

Clone the code and run this command:

```bash
$ cargo install --path .
```
