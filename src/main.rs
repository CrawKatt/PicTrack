mod init;
mod core;
mod commands;
mod commit;

use std::fs::create_dir_all;
use crate::init::init_repo;
use clap::Parser;
use std::path::Path;
use crate::commands::Commands;
use crate::commit::create_commit;
use crate::core::{compare_images, generate_image_hash, load_image, save_image, Cli};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            match init_repo() {
                Ok(()) => println!("Repositorio inicializado correctamente."),
                Err(why) => eprintln!("Error al inicializar el repositorio: {why}"),
            }
        },
        Commands::Add { path } => {
            let img_path = Path::new(path);
            match load_image(&img_path) {
                Ok(image) => {
                    let image_hash = generate_image_hash(&image);
                    let objects_dir = Path::new(".images/objects");
                    create_dir_all(objects_dir).expect("No se pudo crear el directorio de objetos");

                    let image_path = objects_dir.join(format!("{image_hash}.png"));
                    if image_path.exists() {
                        println!("La imagen ya existe en el repositorio con hash: {image_hash}");
                    } else {
                        match save_image(&image, &img_path) {
                            Ok(()) => println!("Imagen guardada en el repositorio."),
                            Err(why) => eprintln!("Error al guardar la imagen: {why}"),
                        }
                    }
                },
                Err(why) => eprintln!("No se pudo cargar la imagen: {why}"),
            }
        },
        Commands::Compare { image1, image2 } => {
            let img1_path = Path::new(image1);
            let img2_path = Path::new(image2);

            let img1 = load_image(&img1_path).expect("No se pudo cargar la primera imagen");
            let img2 = load_image(&img2_path).expect("No se pudo cargar la segunda imagen");

            if compare_images(&img1, &img2) {
                println!("Las imágenes son idénticas.");
            } else {
                println!("Las imágenes son diferentes.");
            }
        },
        Commands::Commit { path, message} => {
            let img_path = Path::new(path);

            match load_image(&img_path) {
                Ok(image) => {
                    // Generar un hash para la imagen
                    let image_hash = generate_image_hash(&image);

                    // Crear el directorio .images/objects si no existe
                    let objects_dir = Path::new(".images/objects");
                    create_dir_all(objects_dir).expect("No se pudo crear el directorio de objetos");

                    // Guardar la imagen con el hash como nombre
                    let image_path = objects_dir.join(format!("{image_hash}.png"));

                    if image_path.exists() {
                        println!("La imagen ya existe en el repositorio con hash: {image_hash}");
                    } else {
                        if let Err(why) = save_image(&image, &image_path) {
                            eprintln!("Error al guardar la imagen: {why}");
                        } else {
                            println!("Imagen guardada en el repositorio con hash: {image_hash}");
                        }

                        // Crear el commit
                        if let Err(why) = create_commit(&image_hash, message) {
                            eprintln!("Error al crear el commit: {why}");
                        } else {
                            println!("Commit creado con éxito.");
                        }
                    }
                },
                Err(why) => eprintln!("No se pudo cargar la imagen: {why}"),
            }
        }
    }
}
