use crate::branch::BRANCHES_DIR;
use crate::current_branch::current_branch;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

pub fn create_commit(hash: &str, message: &str) -> io::Result<()> {
    // Obtener la rama actual
    let branch = current_branch()?;

    // Crear el archivo de commit en la rama correspondiente
    let branch_dir = Path::new(BRANCHES_DIR).join(branch);
    let commit_file_path = branch_dir.join(format!("{hash}.txt"));

    let mut commit_file = File::create(commit_file_path)?;

    let now = chrono::Utc::now();
    writeln!(commit_file, "commit: {hash}")?;
    writeln!(commit_file, "date: {now}")?;
    writeln!(commit_file, "message: {message}")?;

    Ok(())
}