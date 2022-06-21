use clap::{value_t, App, Arg};
use image::{self, GenericImageView, Pixel, Rgba};

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
        .arg(
            Arg::with_name("stride")
                .long("stride")
                .takes_value(true)
                .default_value("1")
                .help("Set the stride")
                .required(false),
        )
        .get_matches();
    let stride = value_t!(matches, "stride", u32).unwrap();
    let input = matches.value_of("INPUT").unwrap();
    let image = image::open(input).expect("Could not open file");
    let (x_size, y_size) = image.dimensions();

    println!("<html><style>body {{font-size: 4px; line-height: 0.6; }}</style><body><pre>");
    for y in (0..y_size).step_by(stride as usize) {
        for x in (0..x_size).step_by(stride as usize) {
            let view_width = stride;
            let view_height = stride;
            let excess_width = if x + view_width > image.width() {
                x + view_width - image.width()
            } else {
                0
            };
            let excess_height = if y + view_height > image.height() {
                y + view_height - image.height()
            } else {
                0
            };

            let sub_image =
                *image.view(x, y, view_width - excess_width, view_height - excess_height);
            let mean = average_pixel(&sub_image);
            let char = u8_to_ascii(mean);
            print!("{}", char);
        }
        println!();
    }
    println!("</pre></body></html>");
}
