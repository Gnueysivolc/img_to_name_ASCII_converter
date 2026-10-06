use crate::{
    converter::print_into_file,
    en_decoder::{decoder, encoder, print_into_file_custom},
};

mod converter;
mod en_decoder;
//
// change this line to change the letters used
//
//
//
//
//
const CLOVIS: [u8; 7] = [b' ', b'l', b'c', b'i', b'v', b's', b'o'];
//let CLOVIS = *b" .`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
//const CLOVIS: [u8; 10] = *b" .:-=+*#%@";
const FILE_TYPE: &str = "clovis";

pub fn run() -> image::ImageResult<()> {
    let (pixels, width, height) = converter::convert_image_into_raw()?;

    let brightness_level: Vec<u8> = converter::raw_to_brightness_level(pixels, &CLOVIS);
    let output: Vec<u8> = converter::brightness_level_to_letter(brightness_level, &CLOVIS);
    let _ = converter::print_into_file(output, width, height);

    Ok(())
}

pub fn encode_and_create_file_run() -> image::ImageResult<()> {
    encode_and_create_file()?;
    decode_and_output_txt();

    Ok(())
}

pub fn just_decode_and_run() {
    decode_and_output_txt();
}

// private function
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//

fn encode_and_create_file() -> image::ImageResult<()> {
    let (pixels, width, height) = converter::convert_image_into_raw()?;
    let brightness_level: Vec<u8> = converter::raw_to_brightness_level(pixels, &CLOVIS);

    let encoded_data = encoder(brightness_level, FILE_TYPE, &CLOVIS, width, height);

    print_into_file_custom(&encoded_data, FILE_TYPE)?;

    Ok(())
}

fn decode_and_output_txt() {
    let (brightness_level, width, height, _file_type) = decoder("result/result.clovis");
    let output: Vec<u8> = converter::brightness_level_to_letter(brightness_level, &CLOVIS);
    let _ = en_decoder::print_into_file_clovis(output, width, height);
}
