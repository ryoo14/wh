use std::env;
use anyhow::Result;

pub fn path() -> Result<String> {
    let who_root = "WHOROOT";
    match env::var(who_root) {
        Ok(wr) => {
           Ok(wr) 
        },
        Err(_) => {
            let home = "HOME";
            let h = env::var(home)?;
            Ok(h + "/who")
        }
    }
}
