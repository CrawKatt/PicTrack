use std::fs;
use std::io;
use std::path::Path;
use crate::commands::branch::BRANCHES_DIR;
use crate::commands::current_branch::current_branch;

pub fn reset_commit(commit_hash: &str) -> io::Result<()> {
    let branch = current_branch()?;
    let branch_dir = Path::new(BRANCHES_DIR).join(&branch);
    let commit_path = branch_dir.join(format!("{commit_hash}.txt"));

    if commit_path.exists() {
        fs::remove_file(commit_path)?;
        println!("Commit {commit_hash} eliminado");
    } else {
        eprintln!("El commit {commit_hash} no existe en la rama actual.");
    }

    Ok(())
}