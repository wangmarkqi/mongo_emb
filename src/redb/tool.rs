use anyhow::Result;
use chrono::Local;
use std;
#[macro_export]
macro_rules! f_str {
    ($($tokens:tt)*) => {
        format!($($tokens)*)
    };
}

pub fn is_win() -> bool {
    std::env::consts::OS == "windows"
}
pub fn create_dirs(d: &str) -> Result<()> {
    std::fs::create_dir_all(d)?;
    Ok(())
}
pub fn now() -> String {
    let date = Local::now();
    let res = date.format("%Y-%m-%d %H:%M:%S");
    f_str!("{res}")
}
#[test]
fn test_tool() {
    dbg!(now());
}
