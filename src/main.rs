use image::imageops::FilterType;
use std::fs;

fn main() -> image::ImageResult<()> {
    let img = image::open("src/image.png")?;

    let columns: u32 = 80;

    let rows = (columns as f64 * img.height() as f64 / img.width() as f64 * 0.5)
        .round()
        .max(1.0) as u32;

    let small = img.resize_exact(columns, rows, FilterType::Triangle);

    let gray = small.into_luma8();
    let (width, height) = gray.dimensions();

    let pixels: Vec<u8> = gray.into_raw();

    let clovis: [u8; 7] = [b' ', b'l', b'c', b'i', b'v', b's', b'o'];

    let output = brightness_to_letter(pixels, &clovis);

    for i in 0..height {
        for j in 0..width {
            let index = (i * width + j) as usize;
            let byte = output[index];
            print!("{}", char::from(byte));
        }
        println!();
    }

    //let haha: Vec<u8> = vec![12, 234, 90, 42, 13, 69, 67, 138, 192, 100];
    //let sad: [u8; 6] = [1, 2, 3, 4, 5, 6];
    //let clovis: [u8; 7] = [b' ', b'l', b'c', b'i', b'v', b's', b'o'];
    //
    //let output = brightness_to_letter(haha, &clovis);
    //
    //println!("{:?}", output);
    //
    //for &byte in &output {
    //    print!("{}", char::from(byte));
    //}
    //
    //

    Ok(())
}

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
