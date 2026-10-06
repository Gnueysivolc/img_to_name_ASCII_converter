use image::imageops::FilterType;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};

// basically collects the argument from terminal running command, then take the path and resolution
// (no. of column) of the image want to output as ASCII, then resize the image and turn it into
// grayscale with the help of image crate
//
//then return the pixels data in grayscale [u8] from 0 to 255, and width and height
pub fn convert_image_into_raw() -> image::ImageResult<(Vec<u8>, u32, u32)> {
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

    Ok((pixels, width, height))
}

// this function takes the output from the final all letter changed vector and print it into a file
// called result.txt
//
// which is regulated by height and width and will output final product of the ASCIII art
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

// this function converts the raw pixel which contains grayscale [u8] 0-255, into brightness level
// classified from 0 to a number which is the length of the name or choosen ASCII character no. to use
//
// then returns the level of brightness in the form of vector which can be used for any array of
// ASCII character will the same length or no. of characters
pub fn raw_to_brightness_level(pixels: Vec<u8>, letters: &[u8]) -> Vec<u8> {
    // the pixels vector should contain 0 to 255 scale of brightness
    // which represents all the pixel in the image
    // propably after downgrading
    assert!(!letters.is_empty(), "letters cannot be empty");

    pixels
        .iter()
        .map(|&grayscale| (usize::from(grayscale) * letters.len() / 256) as u8)
        //.map(|index| letters[index])
        .collect()
}

// this functions change the brightness level vector into letters from the ASCII array given, so
// just substitue the index and change into the letters in form of u8
pub fn brightness_level_to_letter(brightness_level: Vec<u8>, letters: &[u8]) -> Vec<u8> {
    // the pixels vector should contain 0 to 255 scale of brightness
    // which represents all the pixel in the image
    // propably after downgrading
    assert!(!letters.is_empty(), "letters cannot be empty");

    brightness_level
        .iter()
        .map(|&index| letters[usize::from(index)])
        .collect()
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
