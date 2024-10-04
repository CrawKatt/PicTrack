use std::fs;
use std::io;
use std::path::Path;
use crate::current_branch::current_branch;

pub fn status() -> io::Result<()> {
    let branch = current_branch()?;
    println!("En la rama: {branch}");

    let objects_dir = Path::new(".images/objects");
    if !objects_dir.exists() {
        println!("No hay objetos en el repositorio.");
        return Ok(());
    }

    // Listar las imágenes en el repositorio
    println!("\nImágenes en el repositorio:");
    for entry in fs::read_dir(objects_dir)? {
        let entry = entry?;
        if entry.path().is_file() {
            println!("{}", entry.file_name().to_string_lossy());
        }
    }

    Ok(())
}
