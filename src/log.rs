use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

// Función para leer y mostrar el historial de commits
pub fn log_commits() -> io::Result<()> {
    let commits_dir = Path::new(".images/commits");

    // Verificar si el directorio de commits existe
    if !commits_dir.exists() {
        eprintln!("No hay commits en el repositorio.");
        return Ok(());
    }

    // Obtener la lista de archivos de commits
    let mut entries: Vec<_> = fs::read_dir(commits_dir)?
        .filter_map(|entry| entry.ok())
        .collect();

    // Ordenar los archivos por nombre (el hash se usa como nombre)
    entries.sort_by_key(|entry| entry.file_name());

    // Leer y mostrar cada archivo de commit
    for entry in entries {
        let path = entry.path();
        if path.is_file() {
            let file = fs::File::open(&path)?;
            let reader = io::BufReader::new(file);

            println!("\nCommit en {}", path.display());

            // Leer el archivo línea por línea y mostrar su contenido
            for line in reader.lines() {
                let line = line?;
                println!("{}", line);
            }
        }
    }

    Ok(())
}