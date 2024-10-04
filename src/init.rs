// En `init.rs`
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::io;

pub fn init_repo() -> io::Result<()> {
    let repo_path = Path::new(".images");
    let branches_dir = repo_path.join("branches");

    // Verificar si el repositorio ya ha sido inicializado
    if repo_path.exists() {
        eprintln!("El repositorio ya ha sido inicializado.");
        return Ok(());
    }

    // Crear la estructura del repositorio
    fs::create_dir_all(&branches_dir)?;

    // Crear la rama principal (main)
    let main_branch = branches_dir.join("main");
    fs::create_dir_all(&main_branch)?;

    // Apuntar el HEAD a la rama principal
    let mut head_file = File::create(".images/HEAD")?;
    writeln!(head_file, "ref: branches/main")?;

    println!("Repositorio inicializado correctamente.");
    Ok(())
}
