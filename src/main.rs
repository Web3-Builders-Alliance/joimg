use std::fs::File;

use image::ImageBuffer;
use itertools::Itertools;
use serde_derive::{self, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Layout {
    pub assets: Vec<Asset>,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    pub kind: String,
    pub variants: Vec<String>,
    pub position: Point,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

fn main() {
    let json_file_path = "layout.json";
    let file = File::open(json_file_path).unwrap();
    let Layout {
        width,
        height,
        assets,
    } = serde_json::from_reader(file).expect("error while reading json");

    let output_dir = "assets/";

    for asset_combination in generate_combinations(&assets).iter().enumerate() {
        let mut canvas = ImageBuffer::new(width, height);

        let (i, to_conv) = asset_combination;
        for (asset, variant) in assets.iter().zip(to_conv) {
            match asset.kind.as_str() {
                "image" => {
                    let asset_image = image::open(variant).unwrap();
                    image::imageops::overlay(
                        &mut canvas,
                        &asset_image,
                        asset.position.x.into(),
                        asset.position.y.into(),
                    );
                }
                "text" => {
                    // generate text image and overlay it on the canvas
                }
                _ => {}
            }
        }

        let output_path = format!("{}{}.png", output_dir, i);
        std::fs::create_dir_all(output_dir).expect("Failed to create output directory");
        dbg!(&output_path);
        canvas.save(output_path).expect("Failed to save image");

        println!("Image '{}' generated successfully.", i);
    }
}

fn generate_combinations(assets: &[Asset]) -> Vec<Vec<String>> {
    let mut combinations = vec![];
    for asset in assets {
        combinations.push(asset.variants.clone());
    }
    combinations.into_iter().multi_cartesian_product().collect()
}
