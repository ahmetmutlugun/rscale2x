# rscale2x

## Installation

### From releases

#### Linux/MacOS

Download the rscale2x binary from the releases page. Run the binary locally using `rscale2x`. To use the binary system-wide,
place it in `/usr/local/bin` or `$HOME/bin`.

#### Windows

Build from source

### From source

#### 1. Install Dependencies

[Install](https://doc.rust-lang.org/cargo/getting-started/installation.html) cargo and rust.

#### 2. Building

Download the source of the project, then run ```cargo build --release``` in the directory to generate the binary
in `./target/release/`

## Usage

```
Upscale and save images using the scale2x algorithm.

Usage: rust_scale2x [OPTIONS] --input <INPUT> --output <OUTPUT>

Options:
  -i, --input <INPUT>          Input image file path
  -o, --output <OUTPUT>        Output image file path
  -b, --blur <BLUR>            Gaussian blur [default: 0]
  -u, --unsharpen <UNSHARPEN>  Unsharpen [default: 0]
      --hue <HUE>              Hue shift [default: 0]
  -h, --help                   Print help information
  -V, --version                Print version information

```

### Performance

#### Speed

Execution times were recorded using the 8-Core M1 Pro chip.

| | rscale2x  | scale2x |
| ------------- | ------------- | ------------- |
| tree.png (3741x2494) | 3.9s  | 4.5s  |
| grid.png (32x32) | 0.00186s | 0.00153s  |

### Dependencies:

- [Rust and Cargo](http://rust-lang.org/)
- [Image](https://docs.rs/image/0.24.5/image/)
- [Clap](https://docs.rs/clap/4.1.4/clap/)

### Credits:

tree.png: [Unsplash](https://unsplash.com/photos/tGTVxeOr_Rs)