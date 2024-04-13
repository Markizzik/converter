use crate::errors::AppErr;
use axum::http::StatusCode;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::read_dir;
use tokio::process::Command;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const STR_LEN: usize = 16;

#[derive(Serialize, Deserialize, Debug)]
pub enum ImageFileFormat {
    Jpeg,
    Gif,
    Png,
    Svg,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum VideoFileFormat {
    Mp4,
    Webm,
    Mkv,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AudioFileFormat {
    MP3,
    Wav,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DocumentFileFormat {
    Pdf,
    Doc,
    Docx,
    Txt,
    Json,
    Yaml,
    Toml,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CompressionFormat {
    Zip,
    Gzip,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FileFormat {
    Image(ImageFileFormat),
    Video(VideoFileFormat),
    Audio(AudioFileFormat),
    Document(DocumentFileFormat),
    Compression(CompressionFormat),
}

pub fn gen_random_string() -> String {
    let mut rng = rand::thread_rng();

    let rand_str: String = (0..STR_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    rand_str
}

pub fn ext_by_name(path: &str, file_name: &str) -> Result<String, AppErr> {
    let dir = read_dir(path).map_err(|e| {
        AppErr::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("error while read dir: {e}"),
        )
    })?;

    let dir = dir.filter_map(Result::ok);

    for file in dir {
        let p = file.path().to_string_lossy().into_owned();
        if p.contains(file_name) {
            return Ok(file
                .path()
                .extension()
                .ok_or_else(|| {
                    AppErr::new(StatusCode::INTERNAL_SERVER_ERROR, "error while read ext")
                })?
                .to_string_lossy()
                .into_owned());
        }
    }

    Err(AppErr::new(StatusCode::INTERNAL_SERVER_ERROR, "no file"))
}

pub async fn ffmpeg_convert(input_file_path: &str, output_file_path: &str) -> Result<(), AppErr> {
    Command::new("ffmpeg")
        .args(vec![
            "-y",
            "-i",
            &input_file_path,
            "-ac",
            "1",
            "-loglevel",
            "quiet",
            &output_file_path,
        ])
        .status()
        .await
        .map_err(|e| {
            AppErr::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("err while spawn ffmpeg task: {e}"),
            )
        })?;

    Ok(())
}

pub async fn rm_file(file_path: &str) -> Result<(), AppErr> {
    dbg!(file_path);

    Command::new("rm")
        .arg(file_path)
        .status()
        .await
        .map_err(|e| {
            AppErr::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("err while spawn rm task: {e}"),
            )
        })?;

    Ok(())
}
