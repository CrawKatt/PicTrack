use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use crate::current_branch::current_branch;
use crate::branch::BRANCHES_DIR;

// Mostrar el historial de commits de la rama actual
pub fn log_commits() -> io::Result<()> {
    let branch = current_branch()?;
    let branch_dir = Path::new(BRANCHES_DIR).join(&branch);

    if !branch_dir.exists() {
        eprintln!("La rama '{}' no tiene commits.", branch);
        return Ok(());
    }

    // Obtener la lista de archivos de commits en la rama actual
    let mut entries: Vec<_> = fs::read_dir(&branch_dir)?
        .filter_map(|entry| entry.ok())
        .collect();

    // Ordenar los commits por nombre (el hash se usa como nombre)
    entries.sort_by_key(|entry| entry.file_name());

    // Leer y mostrar cada commit
    for entry in entries {
        let path = entry.path();
        if path.is_file() {
            let file = fs::File::open(&path)?;
            let reader = BufReader::new(file);

            println!("\nCommit en {}", path.display());

            // Leer el archivo línea por línea
            for line in reader.lines() {
                let line = line?;
                println!("{}", line);
            }
        }
    }

    Ok(())
}
