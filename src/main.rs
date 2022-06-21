use clap::{App, Arg};
use image::{self, GenericImageView, ImageBuffer, Pixel, Rgb, Rgba};

const STRIDE: u32 = 2;

fn average_pixel(image: &dyn GenericImageView<Pixel = Rgba<u8>>) -> u8 {
    let (x_size, y_size) = image.dimensions();
    let mut mean = 0f32;

    for y in 0..y_size {
        for x in 0..x_size {
            let pixel = image.get_pixel(x, y);
            let chans = pixel.channels();
            for chan in chans {
                mean += (*chan as f32) / ((4 * x_size * y_size) as f32);
            }
        }
    }
    return mean as u8;
}

fn u8_to_ascii(x: u8) -> char {
    match x {
        241..=255 => ' ',
        221..=240 => '.',
        201..=220 => '_',
        181..=200 => ':',
        161..=180 => '+',
        141..=160 => '!',
        121..=140 => '|',
        101..=120 => '[',
        81..=100 => '=',
        61..=80 => '%',
        41..=60 => '#',
        21..=40 => '@',
        0..=20 => '%',
    }
}

fn main() {
    let matches = App::new("bluescii")
        .version("0.1")
        .author("Kabir G. <kabirgoel.kg@gmail.com>")
        .arg(
            Arg::with_name("INPUT")
                .help("Set the input image")
                .required(true)
                .index(1),
        )
        .get_matches();
    let input = matches.value_of("INPUT").unwrap();
    let image = image::open(input).expect("Could not open file");
    let (x_size, y_size) = image.dimensions();

    let mut imgbuf = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(x_size / STRIDE, y_size / STRIDE);

    println!("<html><style>body {{font-size: 4px; line-height: 0.6; }}</style><body><pre>");
    for y in (0..y_size).step_by(STRIDE as usize) {
        for x in (0..x_size).step_by(STRIDE as usize) {
            let sub_image = *image.view(x, y, STRIDE, STRIDE);
            let mean = average_pixel(&sub_image);
            if let Some(pixel) = imgbuf.get_pixel_mut_checked(x / STRIDE, y / STRIDE) {
                *pixel = image::Rgb([mean, mean, mean])
            }
            let char = u8_to_ascii(mean);
            print!("{}", char);
        }
        println!();
    }
    println!("</pre></body></html>");
    imgbuf.save("out.png").unwrap();
}
