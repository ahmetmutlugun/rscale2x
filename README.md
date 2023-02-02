# Rust2x
## Installation
### From releases
####Linux/MacOS
Download the rust2x binary from the releases page. Run the binary locally using `rust2x`. To use the binary system wide, place it in `/usr/local/bin` or `$HOME/bin`.
####Windows
Build from source
### From source
####1. Install Dependencies
[Install](https://doc.rust-lang.org/cargo/getting-started/installation.html) cargo and rust. 
####2. Building
Download the source of the project, then run ```cargo build --release``` in the directory to generate the binary in `./target/release/`
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
###Performance

####Speed
The time elapsed was recorded in seconds by running each binary 100 times through the command line with `/usr/bin/time -hl`. Times were recorded on an M1 Pro Macbook Pro (8 cores 8 threads) and a Ryzen 5 5600 (6 cores 12 threads).


| | Rust2x  | Scale2x | Swift2x |
| ------------- | ------------- | ------------- | ------------- |
| tree.png (3741x2494) | x  | x  | x |
| duck.png (16x15) | x  | x  | x |
####Memory
| | Rust2x  | Scale2x | Swift2x |
| ------------- | ------------- | ------------- | ------------- |
| tree.png (3741x2494) | x  | x  | x |
| duck.png (16x15) | x  | x  | x |
###Dependencies:
- [Rust and Cargo](http://rust-lang.org/)
- [Image](https://docs.rs/image/0.24.5/image/)
- [Clap](https://docs.rs/clap/4.1.4/clap/)
###Credits:
tree.png: [Unsplash](https://unsplash.com/photos/tGTVxeOr_Rs)  
duck.png: [Stardew Valley](https://stardewvalleywiki.com)