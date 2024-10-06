use image_diff::diff;
use std::fs::File;
use std::io;
use image::ImageOutputFormat;

// Función para comparar dos imágenes y generar una imagen de diff
pub fn visual_diff(image1_path: &str, image2_path: &str, output_path: &str) -> io::Result<()> {
    // Cargar las imágenes
    let img1 = image::open(image1_path).expect("No se pudo cargar la primera imagen.");
    let img2 = image::open(image2_path).expect("No se pudo cargar la segunda imagen.");

    // Generar la imagen de diff
    let diff_image = diff(&img1, &img2).expect("Error al generar la imagen de diferencias.");

    // Guardar la imagen de diferencias
    let mut output_file = File::create(output_path)?;
    diff_image.write_to(&mut output_file, ImageOutputFormat::Png)
        .expect("Error al guardar la imagen de diferencias.");

    println!("La imagen de diferencias se guardó en '{}'.", output_path);
    Ok(())
}