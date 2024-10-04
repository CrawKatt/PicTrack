use crate::commands::Commands;
use clap_derive::Parser;
use image::{DynamicImage, GenericImageView, ImageFormat};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::path::Path;

#[derive(Parser)]
#[command(name = "Image Versioner")]
#[command(about = "Sistema de control de versiones para imágenes", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

// Función para guardar una imagen
pub fn save_image(image: &DynamicImage, path: &Path) -> std::io::Result<()> {
    let mut output_file = File::create(path)?;
    image.write_to(&mut output_file, ImageFormat::Png).expect("Error al guardar la imagen");
    Ok(())
}

// Función para cargar una imagen desde un archivo
pub fn load_image(path: &Path) -> Result<DynamicImage, image::ImageError> {
    let img = image::open(path)?;
    Ok(img)
}

pub fn generate_image_hash(image: &DynamicImage) -> String {
    let mut hasher = Sha256::new();
    for pixel in image.pixels() {
        hasher.update(&pixel.2 .0); // Solo usaremos los valores de color
    }
    format!("{:x}", hasher.finalize())
}

// Función para comparar imágenes (usando diffs simplificados)
pub fn compare_images(img1: &DynamicImage, img2: &DynamicImage) -> bool {
    img1.pixels().eq(img2.pixels())
}