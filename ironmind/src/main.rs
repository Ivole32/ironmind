use std::{fs};
use std::fs::DirEntry;
use std::env;

#[derive(Debug)]
struct LableFile {
    magic_number: u32,
    item_count: u32,
    path: DirEntry
}

#[derive(Debug)]
struct ImageFile {
    magic_number: u32,
    item_count: u32,
    row_number: u32,
    column_number: u32,
    path: DirEntry
}

fn convert_u8_to_u32(value: u8) -> u32 {
    return value.into();
}

fn convert_u8_array_to_u32(data: &[u8; 4]) -> u32 {
    let converted: [u32; 4] = [
        convert_u8_to_u32(data[0]),
        convert_u8_to_u32(data[1]),
        convert_u8_to_u32(data[2]),
        convert_u8_to_u32(data[3])
    ];

    return ((converted[0] << 24) + (converted[1] << 16) + (converted[2] << 8) + (converted[3])).into();
}

fn parse_training_files(args: &Vec<String>) -> (LableFile, ImageFile) {
    if args.len() <= 1 {
        panic!("No training folder specified.")
    }
    let training_folder_path = &args[1];
    let paths = fs::read_dir(training_folder_path).unwrap();
    
    let mut files: (Option<LableFile>, Option<ImageFile>) = (None, None);

    let i = 0;
    for path in paths {
        if i > 1 {
            break;
        }

        let raw_data: Vec<u8> = fs::read(path.as_ref().unwrap().path()).unwrap();

        let magic_number_data = [raw_data[0], raw_data[1], raw_data[2], raw_data[3]];
        let magic_number: u32 = convert_u8_array_to_u32(&magic_number_data);

        let item_count_data = [raw_data[4], raw_data[5], raw_data[6], raw_data[7]];
        let item_count: u32 = convert_u8_array_to_u32(&item_count_data);

        if magic_number == 2049 {
            println!("Parsing Lable-File header");
            let lable_file = LableFile {magic_number, item_count, path: path.unwrap()};

            files.0 = Some(lable_file);

        } else if magic_number == 2051 {
            println!("Parsing Image-File header");

            let row_number_data = [raw_data[8], raw_data[9], raw_data[10], raw_data[11]];
            let row_number: u32 = convert_u8_array_to_u32(&row_number_data);

            let column_number_data = [raw_data[12], raw_data[13], raw_data[14], raw_data[15]];
            let column_number: u32 = convert_u8_array_to_u32(&column_number_data);

            let image_file = ImageFile { magic_number, item_count, row_number, column_number, path: path.unwrap() };

            files.1 = Some(image_file);

        } else {
            panic!("Unknown magic bytes: {:?}", magic_number);
        }
    }

    println!("Lable: {:#?}; Image: {:#?}", files.0, files.1);

    return (files.0.expect("Lable file was not loaded"), files.1.expect("Image file was not loaded"));

}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (lables, images) = parse_training_files(&args);
}