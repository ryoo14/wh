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
        // XD
        WalkDir::new(self.root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_dir())
            .filter(|e| e.file_name().to_str().map(|s| s.ends_with(".git")).unwrap_or(false))
            .for_each(|e| workdir_list.push(e.path().display().to_string().replace("/.git", "")));
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
