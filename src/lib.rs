use std::fs;
use std::env;
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
}


