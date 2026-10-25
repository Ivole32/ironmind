use std::{fs};
use std::env;

fn convert_u8_to_u32(value: u8) -> u32 {
    return value.into();
}

fn convert_u8_array_to_u32(data: [u8; 4]) -> u32 {
    let converted: [u32; 4] = [
        convert_u8_to_u32(data[0]),
        convert_u8_to_u32(data[1]),
        convert_u8_to_u32(data[2]),
        convert_u8_to_u32(data[3])
    ];

    return ((converted[0] << 24) + (converted[1] << 16) + (converted[2] << 8) + (converted[3])).into();
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let raw_data: Vec<u8> = fs::read(file_path)?;

    let magic_number_data = [raw_data[0], raw_data[1], raw_data[2], raw_data[3]];
    let item_count_data = [raw_data[4], raw_data[5], raw_data[6], raw_data[7]];

    let magic_number: u32 = convert_u8_array_to_u32(magic_number_data);
    let item_count: u32 = convert_u8_array_to_u32(item_count_data);
        
    println!("Magic Number: {:?}; Item Count: {:?}", magic_number, item_count);

    Ok(())
}