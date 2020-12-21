use anyhow::Result;
use std::{fs::File, io::Read, path::Path, str::FromStr};

pub fn read_lines<T, P>(path: P) -> Result<Vec<T>>
where
    T: FromStr,
    P: AsRef<Path>,
    <T as FromStr>::Err: Into<anyhow::Error>,
{
    let bytes = read_bytes(path)?;

    let contents = String::from_utf8(bytes)?;

    let result: Result<Vec<T>> = contents
        .lines()
        .map(|s| s.parse::<T>().map_err(|x| anyhow::anyhow!(x)))
        .collect();

    result
}

pub fn read_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let file = File::open(path)?;
    Ok(file.bytes().map(|b| b.unwrap()).collect())
}
