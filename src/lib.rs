use anyhow::Result;
use git2::Repository;
use std::{env, fs};
use walkdir::WalkDir;

pub struct WorkHub {
    pub root: String,
}

impl WorkHub {
    pub fn new() -> Result<Self> {
        let wh_root = "WHROOT";
        let wh_root_path = match env::var(wh_root) {
            Ok(path) => path,
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
            .filter(|d| d.path().ends_with(".git") || d.path().ends_with(".wh"))
            .for_each(|e| {
                let dir = e
                    .path()
                    .strip_prefix(&whroot_slash)
                    .unwrap()
                    .parent()
                    .unwrap()
                    .display()
                    .to_string();
                if !workdir_list.contains(&dir) {
                    workdir_list.push(dir);
                }
            });
        Ok(workdir_list)
    }

    pub fn create(self, targetdir_path: &str) -> Result<()> {
        let targetdir_full_path = self.root + "/" + &targetdir_path.to_string() + "/.wh";
        fs::create_dir_all(targetdir_full_path)?;
        Ok(())
    }

    // TODO: clone private repository
    pub fn get(self, repository_path: &str) -> Result<()> {
        let repository_vec: Vec<&str> = repository_path.split('/').collect();
        if repository_vec.len() != 3 {
            panic!("Invalid input. Make sure that the input value satisfies git_repo_manager/author/project");
        }
        let git_repo_url = String::from("https://") + repository_path;
        let targetdir_full_path = self.root + "/" + repository_path;
        if let Err(e) = Repository::clone(&git_repo_url, &targetdir_full_path) {
            panic!("failed to clone: {}", e);
        };
        Ok(())
    }
}
