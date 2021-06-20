use std::fs;
use std::fs::ReadDir;

pub mod root;
pub mod create;
pub mod delete;

// TODO: referctor
pub fn list() -> ReadDir {
    let root_path = root::path().unwrap();
    let hoge = fs::read_dir(root_path);
    hoge.unwrap()
}
