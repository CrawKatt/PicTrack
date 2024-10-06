use image::{DynamicImage, GenericImageView};

// Función para comparar imágenes (usando diffs simplificados)
pub fn compare_images(img1: &DynamicImage, img2: &DynamicImage) -> bool {
    img1.pixels().eq(img2.pixels())
}