use std::{fs, env};
use git2::Repository;
use anyhow::Result;
use walkdir::WalkDir;

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
        let whroot_slash = self.root + "/";
        // XD
        WalkDir::new(&whroot_slash)
            .min_depth(1)
            .max_depth(4)
            .into_iter()
            .filter_map(|a| a.ok())
            .filter(|b| b.file_type().is_dir())
            .filter(|d| d.path().ends_with(".git"))
            .for_each(|e| workdir_list.push(e.path()
                    .strip_prefix(&whroot_slash).unwrap()
                    .parent().unwrap()
                    .display().to_string()));
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
