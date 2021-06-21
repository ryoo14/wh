use std::fs;
use anyhow::Result;

pub mod root;
pub mod create;
pub mod delete;

pub fn list(root_path: &str) -> Result<Vec<String>> {
    let mut workdir_list: Vec<String> = vec![];
    for dir in fs::read_dir(root_path)? {
        let path = dir?.path();
        if path.is_dir() {
            workdir_list.push(path.display().to_string());
        }
    }
    Ok(workdir_list)
}
