use std::{fs, io};
use std::path::Path;
use std::io::Write;

pub fn init_repo() -> io::Result<()> {
    // Directorio principal oculto del repositorio
    let repo_path = Path::new(".images");

    // Verificar si el directorio ya existe
    if repo_path.exists() {
        println!("El repositorio de imágenes ya ha sido inicializado.");
        return Ok(());
    }

    // Crear el directorio .images
    fs::create_dir(repo_path)?;

    // Crear los subdirectorios dentro de .images
    fs::create_dir(repo_path.join("objects"))?;
    fs::create_dir(repo_path.join("commits"))?;

    // Crear un archivo de configuración inicial
    let config_path = repo_path.join("config");
    let mut config_file = fs::File::create(config_path)?;
    writeln!(config_file, "[config]")?;
    writeln!(config_file, "repository_type = image_repo")?;
    writeln!(config_file, "repository_name = my_image_repo")?;
    writeln!(config_file, "author = John Doe")?;

    println!("Repositorio de imágenes inicializado correctamente en .images/");
    Ok(())
}