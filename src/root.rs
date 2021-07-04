use std::env;
use anyhow::Result;

pub fn path() -> Result<String> {
    let wh_root = "WHROOT";
    match env::var(wh_root) {
        Ok(whroot_path) => {
           Ok(whroot_path)
        },
        Err(_) => {
            let home = "HOME";
            let homedir_path = env::var(home)?;
            Ok(homedir_path + "/wh")
        }
    }
}
