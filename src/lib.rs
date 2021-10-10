use std::{fs, env};
use git2::Repository;
use anyhow::Result;

pub struct WorkHub {
    pub root: String,
}

impl WorkHub {
    pub fn new() -> Result<Self> {
        let wh_root = "WHROOT";
        let wh_root_path = match env::var(wh_root) {
            Ok(path) => {
                path 
            },
            Err(_) => {
                let home = "HOME";
                let path = env::var(home)?;
                path + "/wh"
            }
        };
    Ok(Self { root: wh_root_path })
    }

    pub fn list(self) -> Result<Vec<String>> {
        let mut workdir_list: Vec<String> = vec![];
        for workdir in fs::read_dir(self.root)? {
            let workdir_path = workdir?.path();
            if workdir_path.is_dir() {
                workdir_list.push(workdir_path.display().to_string());
            }
        }
        Ok(workdir_list)
    }

    pub fn create(self, targetdir_path: &str) -> Result<()> {
        let targetdir_full_path = self.root + "/" + &targetdir_path.to_string();
        fs::create_dir(targetdir_full_path)?;
        Ok(())
    }

    // TODO: Fix no error when specifying a non-existent repository.
    pub fn get(self, repository_path: &str) -> Result<()> {
        let github_uri = String::from("https://github.com/") + repository_path;
        let mut repository_vec: Vec<&str> = repository_path.split('/').collect();
        if repository_vec.len() != 2 {
            panic!("Invalid input. Make sure that the input value satisfies author/project");
        }
        if let Some(p) = repository_vec.pop() {
            Repository::clone(&github_uri, self.root + "/" + p)?;
        }
        Ok(())
    }
}
