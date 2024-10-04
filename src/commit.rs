use std::fs::{create_dir_all, File};
use std::path::Path;
use chrono::Utc;
use std::io::Write;

pub fn create_commit(hash: &str, message: &str) -> std::io::Result<()> {
    // Crear el archivo de commit en .images/commits
    let commit_dir = Path::new(".images/commits");
    create_dir_all(commit_dir).expect("No se pudo crear el directorio de commits");

    let commit_file_path = commit_dir.join(format!("{}.txt", hash));
    let mut commit_file = File::create(commit_file_path)?;

    // Obtener la fecha y hora actual
    let now = Utc::now();

    writeln!(commit_file, "commit: {hash}")?;
    writeln!(commit_file, "date: {now}")?;
    writeln!(commit_file, "message: {message}")?;

    Ok(())
}