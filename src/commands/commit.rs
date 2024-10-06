use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use image::{DynamicImage, GenericImageView, ImageFormat};
use sha2::{Digest, Sha256};
use crate::commands::branch::BRANCHES_DIR;
use crate::commands::current_branch::current_branch;

pub fn create_commit(hash: &str, message: &str) -> io::Result<()> {
    // Obtener la rama actual
    let branch = current_branch()?;

    // Crear el archivo de commit en la rama correspondiente
    let branch_dir = Path::new(BRANCHES_DIR).join(branch);
    let commit_file_path = branch_dir.join(format!("{hash}.txt"));

    let mut commit_file = File::create(commit_file_path)?;

    let now = chrono::Utc::now();
    writeln!(commit_file, "commit: {hash}")?;
    writeln!(commit_file, "date: {now}")?;
    writeln!(commit_file, "message: {message}")?;

    Ok(())
}

pub fn generate_image_hash(image: &DynamicImage) -> String {
    let mut hasher = Sha256::new();
    for pixel in image.pixels() {
        hasher.update(&pixel.2 .0); // Solo usaremos los valores de color
    }
    format!("{:x}", hasher.finalize())
}

// Función para cargar una imagen desde un archivo
pub fn load_image(path: &Path) -> Result<DynamicImage, image::ImageError> {
    let img = image::open(path)?;
    Ok(img)
}

pub fn save_image(image: &DynamicImage, path: &Path) -> std::io::Result<()> {
    let mut output_file = File::create(path)?;
    image.write_to(&mut output_file, ImageFormat::Png).expect("Error al guardar la imagen");
    Ok(())
}