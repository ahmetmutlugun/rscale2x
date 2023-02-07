#![allow(unused)]
use std::env;
use image::{DynamicImage, ImageDecoder};
use image::GenericImageView;
use image::GenericImage;
use clap::{arg, Parser};
use image::ImageError::Limits;
use image::imageops::FilterType;

/// Upscale an image and save at double the resolution using the scale2x algorithm.
/// Best suited for Pixel Art. Supported formats: PNG, JPEG, GIF, BMP, ICO, TIFF, WebP
#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// Input image file path
    #[arg(short, long)]
    input: String,

    /// Output image file path
    #[arg(short, long)]
    output: String,

    /// Gaussian blur
    #[arg(short, long, default_value_t = 0.0)]
    blur: f32,

    /// Unsharpen
    #[arg(short, long, default_value_t = 0.0)]
    unsharpen: f32,

    /// Hue shift
    #[arg(long, default_value_t = 0)]
    hue: i32,
}

fn main() {
    let args = Args::parse();
    // Get args as a vector
    // Set image path, blur, sharpen and output path
    let blur: f32 = args.blur;
    let sharpen: f32 = args.unsharpen;
    let hue: i32 = args.hue;
    let output_path = args.output;

    //Open image
    let mut img = image::open(args.input).unwrap();

    if blur != 0.0{
        img = img.blur(blur);
    }
    if sharpen != 0.0{
        img = img.unsharpen(sharpen, 1);
    }
    if hue != 0{
        img = img.huerotate(hue);
    }

    // img = img.filter3x3(&[0.0, 0.0, 0.0, 0.0, 1.0, 0.4375, 0.1875, 0.3125, 0.0625]);
    scale2x(img).save(output_path).unwrap();
}

fn scale2x(img: DynamicImage) -> DynamicImage{

    //Set width and height
    let width = img.dimensions().0;
    let height = img.dimensions().1;

    //Create new image
    let mut new_image = DynamicImage::new_rgba16(width*2, height*2);

    //TODO image downscale or noise reduction before upscale
    for pixel in img.pixels() {
        let up = if pixel.1 > 0 { img.get_pixel(pixel.0, pixel.1 - 1) } else { pixel.2 };
        let down = if pixel.1 < height - 1 { img.get_pixel(pixel.0, pixel.1 + 1) } else { pixel.2 };
        let left = if pixel.0 > 0 { img.get_pixel(pixel.0 - 1, pixel.1) } else { pixel.2 };
        let right = if pixel.0 < width - 1 { img.get_pixel(pixel.0 + 1, pixel.1) } else { pixel.2 };

        let mut c1 = img.get_pixel(pixel.0, pixel.1);
        let mut c2 = c1;
        let mut c3 = c1;
        let mut c4 = c1;

        if up != down && left != right {
            c1 = if left == up { left } else { c1 };
            c2 = if up == right { right } else { c2 };
            c3 = if left == down { left } else { c3 };
            c4 = if down == right { right } else { c4 };
        }

        new_image.put_pixel(pixel.0 * 2, pixel.1 * 2, c1);

        if pixel.0 < width {
            new_image.put_pixel(pixel.0 * 2 + 1, pixel.1 * 2, c2);
        }

        if pixel.1 < height {
            new_image.put_pixel(pixel.0 * 2, pixel.1 * 2 + 1, c3);
        }

        if pixel.1 < height && pixel.0 < width {
            new_image.put_pixel(pixel.0 * 2 + 1, pixel.1 * 2 + 1, c4);
        }
    }
    new_image
}

fn horizontal_skip(img: DynamicImage) -> DynamicImage{
    //Set width and height
    let width = img.dimensions().0;
    let height = img.dimensions().1;

    //Create new image
    let mut new_image = DynamicImage::new_rgba16(width*2, height*2);

    // img = img.resize(width*2,height*2, FilterType::Nearest);
    //dynamic_map!(*self, ref p => imageops::resize(p, nwidth, nheight, filter))
    //TODO image downscale or noise reduction before upscale

    for pixel in img.pixels() {
        //TODO implement scaling
        // For now, 1 to 1 scaling is applied.
        if pixel.0 > 0{
            new_image.put_pixel(pixel.0 * 2 - 1, pixel.1 * 2, pixel.2);

        }

        if pixel.0 < width{
            new_image.put_pixel(pixel.0 * 2 + 1, pixel.1 * 2, pixel.2);
        }

        if pixel.1 > 0{
            new_image.put_pixel(pixel.0 * 2 + 1, pixel.1 * 2 - 1, pixel.2);
        }

        if pixel.1 < height{
            new_image.put_pixel(pixel.0 * 2 + 1, pixel.1 * 2 + 1, pixel.2);
        }
    }

    new_image
}