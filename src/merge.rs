use std::fs;
use std::io;
use std::path::Path;
use crate::branch::BRANCHES_DIR;
use crate::current_branch::current_branch;

pub fn merge_branch(soure_branch: &str) -> io::Result<()> {
    let current_branch_name = current_branch()?;
    let current_branch_path = Path::new(BRANCHES_DIR).join(&current_branch_name);
    let source_branch_path = Path::new(BRANCHES_DIR).join(soure_branch);

    if !source_branch_path.exists() {
        eprintln!("La rama '{soure_branch}' no existe");
        return Ok(())
    }

    for entry in fs::read_dir(&source_branch_path)? {
        let entry = entry?;
        let dest = current_branch_path.join(entry.file_name());

        if !dest.exists() {
            fs::copy(entry.path(), dest)?;
            println!("Commit {} fusionado desde la rama '{}'.", entry.file_name().to_string_lossy(), soure_branch);
        }
    }

    Ok(())
}