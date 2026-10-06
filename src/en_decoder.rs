use std::fs::read;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

// encode the raw_data which includes the degree of brightness classified into the Letters.len()
// level
// raw_data encode by adding information into the header which is mentioned below
// then the raw_data part is encoded by an additional algorithm, in this case is XOR
// then combine and whole vector is returned
pub fn encoder(
    raw_data: Vec<u8>,
    file_type: &str,
    letters: &[u8],
    width: u32,
    height: u32,
) -> Vec<u8> {
    //encode the raw real information vector which contains
    //the brightness degree of each pixel
    //
    //add header information to the vector, encode it with a custom
    //algo and then return the vec, which propably will be passed
    //to the write_into_file function

    // header:
    // header.len(), 6(clovis.len()), clovis, version (which is 1 here), width,
    // height, " ", "l", "c", "i", "v", "s", "o"
    let mut header: Vec<u8> = Vec::new();

    header.push(file_type.len() as u8);

    for letter in file_type.bytes() {
        header.push(letter);
    }

    let version: u8 = 1;
    header.push(version);

    header.extend_from_slice(&width.to_le_bytes());
    header.extend_from_slice(&height.to_le_bytes());

    for letter in letters {
        header.push(*letter);
    }

    let encode_algo = encode_xor;
    let encoded_raw_data = encode_algo(raw_data, 67);

    // to show where the header ends
    header.insert(0, header.len() as u8 + 1);

    let mut file_data = header;
    file_data.extend_from_slice(&encoded_raw_data);

    file_data
}

// decoder basically takes the file_path which contains the name of the file_type, and then compare
// it to the name embedded in the .name file header, if same then continue, or else panic
// and then take back all the data out from the header and decode the raw_data in this case by
// decode_xor()
//
// then also return the height and width for later print, the file type is not used right now
pub fn decoder(file_path: &str) -> (Vec<u8>, u32, u32, &str) {
    //return the output, width, height, file type
    //
    //
    //
    // header:
    // header.len(), 6(clovis.len()), clovis, version (which is 1 here), width,
    // height, " ", "l", "c", "i", "v", "s", "o"

    let extension = std::path::Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap();

    let bytes: Vec<u8> = std::fs::read(file_path).expect("no such file path");

    let header_size = bytes[0] as usize;
    let file_type_len = bytes[1] as usize;
    let file_type = &bytes[2..(2 + file_type_len)];

    let version_pos = 2 + file_type_len;
    let _version = bytes[version_pos];

    let width_start = version_pos + 1;
    let height_start = width_start + 4;

    let width = u32::from_le_bytes(bytes[width_start..width_start + 4].try_into().unwrap());

    let height = u32::from_le_bytes(bytes[height_start..height_start + 4].try_into().unwrap());

    let encoded_raw_data = bytes[header_size..].to_vec();

    if extension.as_bytes() != file_type {
        panic!("wrong file format or file corrupted")
    };

    let output = decode_xor(encoded_raw_data, 67);

    (output, width as u32, height as u32, extension)
}

// this function directly prints the whole data vector from the encoder() function into a file to
// create e.g. a .clovis file
pub fn print_into_file_custom(file_data: &[u8], file_type: &str) -> std::io::Result<()> {
    std::fs::write(format!("result/result.{}", file_type), file_data)
}

// this function is basically same as print_into_file in converter.rs but i changed the export part
// and name to classify this is the result specifically for .clovis file
pub fn print_into_file_clovis(
    output: Vec<u8>,
    width: u32,
    height: u32,
) -> Result<(), std::io::Error> {
    let file = File::create("result/result_clovis.txt")?;
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
//
//

// these are the encode and decoder for XOR algorithm
fn encode_xor(file_data: Vec<u8>, key: u8) -> Vec<u8> {
    file_data.iter().map(|&byte| byte ^ key).collect()
}

fn decode_xor(file_data: Vec<u8>, key: u8) -> Vec<u8> {
    file_data.iter().map(|&byte| byte ^ key).collect()
}
