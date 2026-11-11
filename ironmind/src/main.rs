use std::io::Write;
use std::fs;
use std::fs::File;
use std::fs::DirEntry;
use std::env;

#[derive(Debug)]
struct LableFile {
    magic_number: u32,
    item_count: u32,
    raw_data: Vec<u8>,
    data_offset: u32,
    path: DirEntry
}

#[derive(Debug)]
struct ImageFile {
    magic_number: u32,
    item_count: u32,
    row_number: u32,
    column_number: u32,
    raw_data: Vec<u8>,
    data_offset: u32,
    path: DirEntry
}

impl ImageFile {
    fn parse_image(&self, index: u32) -> &[u8] {
        let image_unit: u32 = self.column_number * self.row_number;
        let start: usize = (self.data_offset + index * image_unit).try_into().unwrap();
        let end: usize = (self.data_offset + (index + 1) * image_unit).try_into().unwrap();

        return &self.raw_data[start..end];
    }
}

impl LableFile {
    fn parse_label(&self, index: u32) -> u8 {
        let index: usize = (self.data_offset + index).try_into().unwrap();

        return self.raw_data[index];
    }
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

fn convert_u32_to_u8_array(data: &u32) -> [u8; 4] {
    println!("{:?}", data >> 24);
    println!("{:?}", data >> 16);
    println!("{:?}", data >> 8);
    println!("{:?}", data << 24 >> 24);

    let data: [u8; 4] = [
        (data >> 24).try_into().unwrap(),
        (data >> 16).try_into().unwrap(),
        (data >> 8).try_into().unwrap(),
        (data << 24 >> 24).try_into().unwrap()
    ];

    return data;
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
            let data_offset: u32 = 8;

            let lable_file = LableFile {magic_number, item_count, raw_data, data_offset, path: path.unwrap()};

            files.0 = Some(lable_file);

        } else if magic_number == 2051 {
            println!("Parsing Image-File header");

            let row_number_data = [raw_data[8], raw_data[9], raw_data[10], raw_data[11]];
            let row_number: u32 = convert_u8_array_to_u32(&row_number_data);

            let column_number_data = [raw_data[12], raw_data[13], raw_data[14], raw_data[15]];
            let column_number: u32 = convert_u8_array_to_u32(&column_number_data);

            let data_offset: u32 = 16;

            let image_file = ImageFile { magic_number, item_count, row_number, column_number, raw_data, data_offset, path: path.unwrap() };

            files.1 = Some(image_file);

        } else {
            panic!("Unknown magic bytes: {:?}", magic_number);
        }
    }

    // println!("Lable: {:#?}; Image: {:#?}", files.0, files.1);

    return (files.0.expect("Lable file was not loaded"), files.1.expect("Image file was not loaded"));

}

fn open_file(filepath: &'static str) -> File {
    let file = File::options()
        .append(true)
        .create(true)
        .open(filepath);

    return file.unwrap();
}

fn append_to_file(mut file: &File, buf: &[u8]) {
    file.write(buf).expect("Unable to write data to file");
}

fn write_file_header(file: &File, magic_number: [u8; 4], input_count: u32, output_count: u32) {
    append_to_file(file, &magic_number);
    append_to_file(file, &convert_u32_to_u8_array(&input_count));
    append_to_file(file, &convert_u32_to_u8_array(&output_count));
}

fn train(input_count: u32, output_count: u32, lables: LableFile, images: ImageFile, file: &File) {
    let input_count: usize = input_count.try_into().unwrap();
    let output_count: usize = output_count.try_into().unwrap();

    // A vec that holds another vec for every possible inputs weight
    let mut model_data: Vec<Vec<u32>> = vec![vec![0; output_count]; input_count];

    let mut i = 0;

    loop {
        if i >= lables.item_count{
            break;
        }

        let number: usize = lables.parse_label(i).try_into().unwrap();
        let image: &[u8] = images.parse_image(i);
        //println!("{:?}", number);

        let mut px_count: usize = 0;
        for pixel in image {
            //println!("{:?}", pixel);

            if pixel > &0 {
                model_data[px_count][number] += 1;
            }

            px_count += 1;
        }

        i += 1;
    }

    println!("{:?}; {:?}", model_data, model_data.len());

    for pixel in model_data {
        for weight in pixel.iter() {
            append_to_file(file, &convert_u32_to_u8_array(&weight));
        }
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (lables, images) = parse_training_files(&args);

    let model_file = open_file("test.model");

    // The magic bytes, chose that randomly, this is a representation of 2050
    const MODEL_MAGIC_BYTES: [u8; 4] = [0, 0, 8, 2];

    // Get input neurone count
    let input_count: u32 = images.column_number * images.row_number;

    // The count of possible outputs the model can have, 0-9 in this case
    let output_count: u32 = 10;

    // Write the header values
    write_file_header(&model_file, MODEL_MAGIC_BYTES, input_count, output_count);

    //println!("{:?}", images.parse_image(0));

    println!("{:?}; {:?}", lables.item_count, images.item_count);

    train(input_count, output_count, lables, images, &model_file);


    println!("{:?}", args);
}