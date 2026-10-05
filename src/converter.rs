use image::imageops::FilterType;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn convert_image() -> image::ImageResult<(Vec<u8>, u32, u32)> {
    // take in 2 argument that is the file path of the image
    // and no. of rows (which represents the resolution)
    let args: Vec<String> = env::args().collect();

    let file_path: &str = if args.len() > 1 {
        println!("the file path is {}", args[1]);
        args[1].as_str()
    } else {
        "src/image.png"
    };

    let columns: u32 = if args.len() > 2 {
        println!("the column length would be {}", args[2]);
        args[2]
            .parse::<u32>()
            .expect("column length should be non-negative integer")
    } else {
        500
    };

    let img = image::open(file_path)?;
    let rows = (columns as f64 * img.height() as f64 / img.width() as f64 * 0.5)
        .round()
        .max(1.0) as u32;

    let small = img.resize_exact(columns, rows, FilterType::Triangle);

    let gray = small.into_luma8();
    let (width, height) = gray.dimensions();

    let pixels: Vec<u8> = gray.into_raw();

    // change this line to change the letters used
    //
    //
    //
    //let clovis: [u8; 7] = [b' ', b'l', b'c', b'i', b'v', b's', b'o'];
    //let clovis = *b" .`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
    let clovis = *b" .:-=+*#%@";
    //
    //
    //
    //
    //

    let output: Vec<u8> = brightness_to_letter(pixels, &clovis);

    Ok((output, width, height))
}

pub fn print_into_file(output: Vec<u8>, width: u32, height: u32) -> Result<(), std::io::Error> {
    let file = File::create("result.txt")?;
    let mut writer = BufWriter::new(file);

    for i in 0..height {
        for j in 0..width {
            let index = (i * width + j) as usize;
            let byte = output[index];
            write!(writer, "{}", char::from(byte))?;
        }

        writeln!(writer)?;
    }

    writer.flush()?;

    Ok(())
}

//
//
//
//
//
//
//
//
// private functions
//
//
//
//
//
//
//
//
//

fn brightness_to_letter(pixels: Vec<u8>, letters: &[u8]) -> Vec<u8> {
    // the pixels vector should contain 0 to 255 scale of brightness
    // which represents all the pixel in the image
    // propably after downgrading
    assert!(!letters.is_empty(), "letters cannot be empty");

    pixels
        .iter()
        .map(|&grayscale| usize::from(grayscale) * letters.len() / 256)
        .map(|index| letters[index])
        .collect()
}
