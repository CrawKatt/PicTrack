use std::fs::{self};
use std::io::{self};
use std::path::Path;
use crate::commands::current_branch::current_branch;

pub const BRANCHES_DIR: &str = ".images/branches";
pub const HEAD_FILE: &str = ".images/HEAD";

// Crear una nueva rama o listar las ramas existentes
pub fn handle_branch(name: Option<String>) -> io::Result<()> {
    let branches_dir = Path::new(BRANCHES_DIR);

    // Si no se proporciona un nombre de rama, listar las ramas existentes
    if let Some(branch_name) = name {
        let new_branch_path = branches_dir.join(&branch_name);
        if new_branch_path.exists() {
            eprintln!("La rama '{}' ya existe.", branch_name);
            return Ok(());
        }

        // Obtener la rama actual y copiar su contenido a la nueva rama
        let current_branch_name = current_branch()?;
        let current_branch_path = branches_dir.join(&current_branch_name);

        // Copiar todos los commits de la rama actual a la nueva rama
        fs::create_dir_all(&new_branch_path)?;
        for entry in fs::read_dir(&current_branch_path)? {
            let entry = entry?;
            let dest = new_branch_path.join(entry.file_name());
            fs::copy(entry.path(), dest)?;
        }

        println!("Rama '{}' creada desde '{}'.", branch_name, current_branch_name);
    } else {
        // Listar ramas existentes
        println!("Ramas disponibles:");
        for entry in fs::read_dir(branches_dir)? {
            let entry = entry?;
            if entry.path().is_dir() {
                println!("{}", entry.file_name().to_string_lossy());
            }
        }
    }

    Ok(())
}
