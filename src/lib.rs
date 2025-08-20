#![allow(non_snake_case)]

use std::path::Path;
use anyhow::Result;
use tokio::process::Command;

pub async fn darkprince_lover(url: &str, format: &str, output_path: &str) -> Result<()> {
    let path = Path::new(output_path);
    if !path.exists() {
        tokio::fs::create_dir_all(path).await?;
    }

    let is_playlist = url.contains("list=") || url.contains("playlist");
    let mut command = Command::new("yt-dlp");
    
    command.arg(url)
        .arg("--extract-audio")
        .arg("--audio-format").arg(format);

    if is_playlist {
        // если скачивается плейлист, юзать свою систему директорий
        command.arg("-o").arg(format!("{}/%(playlist_title)s/%(title)s.%(ext)s", output_path));
        command.arg("--yes-playlist");
        println!("ТЕМНЫЙ ПРИНЦ (плейлист)");
    } else {
        command.arg("-o").arg(format!("{}/%(title)s.%(ext)s", output_path));
        command.arg("--no-playlist");
        println!("Web2.0 (одиночное видео)");
    }

    println!("Выполняем команду: {:?}", command);

    let status = command.status().await?;

    if status.success() {
        println!("Мой пиджак и мои штаны успешно скачаны в {}", output_path);
        Ok(())
    } else {
        anyhow::bail!("Ошибка скачивания ТЕМНЫЙ ПРИНЦ. Код выхода: {}", status);
    }
}

pub fn get_current_dir() -> Result<String> {
    std::env::current_dir()?
        .to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow::anyhow!("ТЕМНЫЙ ПРИНЦ не получил директорию"))
}