use std::env;
use anyhow::Result;

pub fn path() -> Result<String> {
    let wh_root = "WHROOT";
    match env::var(wh_root) {
        Ok(wr) => {
           Ok(wr) 
        },
        Err(_) => {
            let home = "HOME";
            let h = env::var(home)?;
            Ok(h + "/wh")
        }
    }
}
