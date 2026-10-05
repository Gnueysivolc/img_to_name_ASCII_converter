mod converter;

pub fn run() -> image::ImageResult<()> {
    let (output, width, height) = converter::convert_image()?;
    let _ = converter::print_into_file(output, width, height);

    Ok(())
}
