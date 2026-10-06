use unique_file_type::{encode_and_create_file_run, just_decode_and_run, run};

fn main() -> image::ImageResult<()> {
    //run()

    //encode_and_create_file_run()?;

    just_decode_and_run();

    Ok(())
}

//
//
//
//
//
//
//
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
