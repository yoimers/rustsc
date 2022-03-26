use std::{error::Error, fs::File, io::Write};

pub async fn download_zip(url: &str, title: &str) -> Result<(), Box<dyn Error>> {
    let img_bytes = reqwest::get(url).await?.bytes().await?;
    let mut file = File::create(title)?;
    file.write_all(&img_bytes)?;
    Ok(())
}
