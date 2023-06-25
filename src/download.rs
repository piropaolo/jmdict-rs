use std::io::{Cursor, Read};

use flate2::read::GzDecoder;
use tar::Archive;

const TARGET: &str = "https://github.com/scriptin/jmdict-simplified/releases/download/3.5.0%2B20230619121907/jmdict-eng-common-3.5.0+20230619121907.json.tgz";

pub fn download_jm_dict() -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(TARGET)?;
    let content = Cursor::new(response.bytes()?);

    let tar = GzDecoder::new(content);
    let mut archive = Archive::new(tar);

    let mut entry = archive.entries()?.next().unwrap()?;

    let mut res = String::new();
    entry.read_to_string(&mut res)?;

    Ok(res)
}