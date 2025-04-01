use image::{DynamicImage, GenericImageView, Rgba};

fn main() {
    let input_path = std::env::args()
        .nth(1)
        .expect("No input and output path provided");
    let output_path = std::env::args().nth(2).expect("No output path provided");

    let img = image::open(input_path).expect("Failed to open image");
    let trimmed = trim_white_bottom(&img);
    save_as_webp(&trimmed, &output_path);
}

fn trim_white_bottom(img: &DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut trim_height = height;

    for y in (0..height).rev() {
        if (0..width).any(|x| is_white(img.get_pixel(x, y))) {
            trim_height = y;
        } else {
            break;
        }
    }

    img.crop_imm(0, 0, width, trim_height)
}

fn is_white(pixel: Rgba<u8>) -> bool {
    pixel.0[0] == 255 && pixel.0[1] == 255 && pixel.0[2] == 255 && pixel.0[3] == 255
}

fn save_as_webp(img: &DynamicImage, output_path: &str) {
    let rgba8 = img.to_rgba8();
    let encoder = webp::Encoder::from_rgba(rgba8.as_raw(), img.width(), img.height());
    let webp_data = encoder.encode(60.0);

    std::fs::write(output_path, &*webp_data).expect("Failed to save WebP image");
}
