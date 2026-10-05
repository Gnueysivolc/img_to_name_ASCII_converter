mod converter;

pub fn run() -> image::ImageResult<()> {
    //
    // change this line to change the letters used
    //
    //
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
    //
    let (pixels, width, height) = converter::convert_image_into_raw()?;

    let brightness_level: Vec<u8> = converter::raw_to_brightness_level(pixels, &clovis);
    let output: Vec<u8> = converter::brightness_level_to_letter(brightness_level, &clovis);
    let _ = converter::print_into_file(output, width, height);

    Ok(())
}
