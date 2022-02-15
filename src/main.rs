mod args;
use args::Args;
use image::{
    imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};
use std::{fs::File, io::BufReader, result::Result};

#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormats,
    BufferTooSmall,
    // UnableToReadImageFromPath(std::io::Error),
    // UnableToFormatImage(String),
    // UnableToDecodeImage(ImageError),
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();
    // match Reader::open(path) {
    //     Ok(image_reader) => {
    //         if let Some(image_format) = image_reader.format() {
    //             match image_reader.decode() {
    //                 Ok(image) => Ok((image, image_format)),
    //                 Err(e) => Err(ImageDataErrors::UnableToDecodeImage(e)),
    //             }
    //         } else {
    //             return Err(ImageDataErrors::UnableToFormatImage(path));
    //         }
    //         (image, image_format)
    //     }
    //     Err(e) => Err(ImageDataErrors::UnableToReadImageFromPath(e)),
    // }
    (image, image_format)
}

fn get_smallest_dimensions(dim1: (u32, u32), dim2: (u32, u32)) -> (u32, u32) {
    // accessing the elements in the tuple, dim1.0 means first element, dim1.1 means second element
    let pix1 = dim1.0 * dim1.1;
    let pix2 = dim2.0 * dim2.1;
    if pix1 < pix2 {
        {
            dim1
        }
    } else {
        {
            dim2
        }
    }
}

fn standarize_size(image1: DynamicImage, image2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimensions(image1.dimensions(), image2.dimensions());
    println!("width: {}, height: {}\n", width, height);

    if image2.dimensions() == (width, height) {
        (image1.resize_exact(width, height, Triangle), image2)
    } else {
        (image1, image2.resize_exact(width, height, Triangle))
    }
}

struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FloatingImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = height * width * 4;
        let buffer = Vec::with_capacity(buffer_capacity.try_into().unwrap()); // try_into().unwrap()
        FloatingImage {
            width,
            height,
            data: buffer,
            name,
        }
    }

    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity() {
            (ImageDataErrors::BufferTooSmall);
        }
        self.data = data;
        Ok(())
    }
}

fn combine_images(image1: DynamicImage, image2: DynamicImage) -> Vec<u8> {
    let vec1 = image1.to_rgb8().into_vec();
    let vec2 = image2.to_rgb8().into_vec();

    alternate_pixels(vec1, vec2)
}

fn alternate_pixels(vec1: Vec<u8>, vec2: Vec<u8>) -> Vec<u8> {
    // if vec1.len = 5, [0, 0, 0, 0, 0]
    let mut combined_data = vec![0u8; vec1.len()];

    let mut i = 0;
    while i < vec1.len() {
        if i % 8 == 0 {
            combined_data.splice(i..=i + 3, set_rgba(&vec1, i, i + 3));
        } else {
            combined_data.splice(i..=i + 3, set_rgba(&vec2, i, i + 3));
        }
        i += 4;
    }

    combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rbga = Vec::new();
    for i in start..=end {
        let val: u8 = match vec.get(i) {
            Some(d) => *d,
            None => panic!("Index out of bounds!"),
        };
        rbga.push(val);
    }

    rbga
}

fn main() -> Result<(), ImageDataErrors> {
    let args = Args::new();
    let (image1, image_format1) = find_image_from_path(args.image1);
    let (image2, image_format2) = find_image_from_path(args.image2);

    if image_format1 != image_format2 {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    let (image1, image2) = standarize_size(image1, image2);
    let mut output = FloatingImage::new(image1.width(), image1.height(), args.output);

    let combined_data = combine_images(image1, image2);
    output.set_data(combined_data)?;

    image::save_buffer_with_format(
        output.name,
        &output.data,
        output.width,
        output.height,
        image::ColorType::Rgb8,
        image_format1,
    )
    .unwrap();
    Ok(())
}

// impl String {
//     fn new() -> Self {
//         String {
//             vec: Vec::new(),
//         }
//     }
// }
