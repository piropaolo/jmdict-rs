use std::io::{Cursor, Read};

use flate2::read::GzDecoder;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use tar::Archive;

const LATEST: &str = "https://api.github.com/repos/scriptin/jmdict-simplified/releases/latest";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Release {
    pub id: u32,
    pub name: String,
    pub assets: Vec<Asset>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Asset {
    pub id: u32,
    pub name: String,
    pub size: u32,
    pub browser_download_url: String,
}

fn get_latest_release_url() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(LATEST)
        .header(USER_AGENT, "jmdict-rs")
        .send();

    let release = response?.json::<Release>()?;

    let download_url = release
        .assets
        .iter()
        .find(|a|
            a.name.contains("jmdict-eng") && a.name.contains("common")
        )
        .map(|a| a.browser_download_url.clone())
        .unwrap();

    Ok(download_url)
}

pub fn download_jm_dict() -> Result<String, Box<dyn std::error::Error>> {
    let latest_url = get_latest_release_url()?;

    let response = reqwest::blocking::get(latest_url)?;
    let content = Cursor::new(response.bytes()?);

    let tar = GzDecoder::new(content);
    let mut archive = Archive::new(tar);

    let mut entry = archive.entries()?.next().unwrap()?;

    let mut res = String::new();
    entry.read_to_string(&mut res)?;

    Ok(res)
}