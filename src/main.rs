use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use serde_json::{Result, Value};

type BaseTitle = String;
// type ImageWidth = u16;
// type FileName = String; // TODO - make a path?

#[derive(Serialize, Deserialize, Debug)]
struct ImageData {
    #[serde(flatten)]
    images: HashMap<BaseTitle, usize>,
}

// #[derive(Serialize, Deserialize)]
// struct SingleImageData {
//     #[serde(flatten)]
//     image_data: HashMap<ImageWidth, TypeVariantData>,
// }

// #[derive(Serialize, Deserialize)]
// struct TypeVariantData {
//     variant_names: HashMap<OutputTypes, FileName>,
// }

// // need to be able to compare key equality
// #[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
// enum OutputTypes {
//     JPG,
//     WEBP,
//     PNG,
//     GIF,
//     SVG,
// }

fn main() {
    let images: Vec<String> = (1..6)
        .map(|val| String::from(format!("file-{}", val)))
        .collect();
    // let widths = [800, 1200, 1800, 2400];
    // let output_types = [OutputTypes::JPG, OutputTypes::WEBP, OutputTypes::PNG];

    let mut output_data = ImageData {
        images: HashMap::new(),
    };

    images.iter().enumerate().for_each(|(i, image_name)| {
        output_data.images.insert(image_name.clone(), i);
    });

    println!("The data:");
    println!("{:?}", images);
    println!("{:?}", output_data);
}
