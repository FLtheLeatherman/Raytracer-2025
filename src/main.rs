use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

mod vec3;
mod color;
use color::Color;

fn main() {
    let path = std::path::Path::new("output/book1/image1.png");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");
    
    let width = 256;
    let height = 256;
    // different from the book, we use image crate to create a .png image rather than outputting .ppm file, which is not widely used.
    // anyway, you may output any image format you like.
    let mut img: RgbImage = ImageBuffer::new(width, height);
    
    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };
    
    for j in (0..height) {
        for i in 0..width {
            let r: f64 = (i as f64) / ((width - 1) as f64) * 255.999;
            let g: f64 = (j as f64) / ((height - 1) as f64) * 255.999;
            let b: f64 = 0.25 * 255.999;
            let pixel_color: Color = Color::new(r, g, b);
            color::write_color(i, j, &pixel_color, &mut img);
        }
        progress.inc(1);
    }
    progress.finish();
    
    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    img.save(path).expect("Cannot save the image to the file");
}