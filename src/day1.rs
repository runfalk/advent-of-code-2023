use anyhow::Result;
use std::fs::File;
use std::path::Path;

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let _file = File::open(path)?;
    Ok((0, None))
}
