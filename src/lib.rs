use std::fs;
use anyhow::Result;

pub mod root;

// TODO: implement
pub mod delete;

pub fn list(whroot_path: &str) -> Result<Vec<String>> {
    let mut workdir_list: Vec<String> = vec![];
    for workdir in fs::read_dir(whroot_path)? {
        let workdir_path = workdir?.path();
        if workdir_path.is_dir() {
            workdir_list.push(workdir_path.display().to_string());
        }
    }
    Ok(workdir_list)
}

pub fn create(targetdir_fullpath: &str) -> Result<()> {
    fs::create_dir(targetdir_fullpath)?;
    Ok(())
}
