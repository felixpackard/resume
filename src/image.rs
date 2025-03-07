use std::{fs, io::Read};

use anyhow::Context;
use data_url::DataUrl;
use image::DynamicImage;
use url::Url;

pub fn fetch_image(url: &String) -> anyhow::Result<DynamicImage> {
    let parsed_url = Url::parse(url).context("Failed to parse image URL")?;

    let data = match parsed_url.scheme() {
        "http" | "https" => {
            let response = ureq::get(url)
                .call()
                .context("Failed to read HTTP(S) response")?
                .into_body()
                .into_reader();
            let mut data = Vec::new();
            response
                .take(10_000_000)
                .read_to_end(&mut data)
                .context("Failed to read HTTP(S) response")?;
            data
        }
        "file" => {
            let path = parsed_url
                .to_file_path()
                .map_err(|_| anyhow::anyhow!("Invalid file URL"))?;
            fs::read(path).context("Failed to read file")?
        }
        "data" => {
            let data_url = DataUrl::process(url).context("Invalid data URL")?;
            let (bytes, _) = data_url
                .decode_to_vec()
                .context("Failed to decode bas64 data")?;
            bytes
        }
        _ => anyhow::bail!("Unsupported URL scheme: {}", parsed_url.scheme()),
    };

    let image = image::load_from_memory(&data).context("Failed to decode image")?;
    Ok(image)
}
