use directories::ProjectDirs;
use futures::stream::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest;
use std::fs::File;
use std::io::Write;

pub async fn download_model() -> Result<String, Box<dyn std::error::Error>> {
    const MODEL_NAME: &str = "SDv1-4.bin";
    let project_dirs = ProjectDirs::from("com", "nekanat", "stablediffusion-wgpu")
        .ok_or("Failed to get project dirs")?;
    let cache_dir = project_dirs.cache_dir();
    let model_dir = cache_dir.join("models");
    std::fs::create_dir_all(&model_dir)?;

    let model_path = model_dir.join(MODEL_NAME);
    if model_path.exists() {
        return Ok(model_path.to_str().unwrap().to_string());
    }

    let url = format!(
        "https://huggingface.co/Gadersd/Stable-Diffusion-Burn/resolve/main/V1/{}",
        MODEL_NAME
    );
    println!("Downloading model from {}...", url);
    download_with_progress(&url, &model_path.to_str().unwrap()).await?;

    Ok(model_path.to_str().unwrap().to_string())
}

async fn download_with_progress(url: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let total_size = response
        .content_length()
        .ok_or("Failed to get content length")?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    let mut file = File::create(path)?;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
        pb.inc(chunk.len() as u64);
    }

    pb.finish_with_message("Download completed");

    Ok(())
}
