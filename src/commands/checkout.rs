use std::fs::File;
use std::io;
use std::path::Path;
use std::io::Write;
use crate::commands::branch::{BRANCHES_DIR, HEAD_FILE};

pub fn checkout_branch(branch_name: &str) -> io::Result<()> {
    let branch_path = Path::new(BRANCHES_DIR).join(branch_name);

    if !branch_path.exists() {
        eprintln!("La rama '{}' no existe.", branch_name);
        return Ok(());
    }

    // Actualizar el archivo HEAD para apuntar a la nueva rama
    let mut head_file = File::create(HEAD_FILE)?;
    writeln!(head_file, "ref: branches/{}", branch_name)?;

    println!("Cambiado a la rama '{}'.", branch_name);
    Ok(())
}