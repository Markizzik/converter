use crate::{app_data::AppData, errors::AppErr};
use axum::{extract::multipart::Field, http::StatusCode};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{path::Path, str::FromStr};
use tokio::{
    fs::{remove_file, File},
    io::AsyncWriteExt,
    process::Command,
};

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const STR_LEN: usize = 20;

#[derive(Deserialize)]
pub struct ConvertQuery {
    pub new_file_format: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ImageFileFormat {
    Gif,
    Jpeg,
    Png,
    Svg,
}

impl FromStr for ImageFileFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        match s.as_str() {
            "gif" => Ok(Self::Gif),
            "jpeg" | "jpg" => Ok(Self::Jpeg),
            "png" => Ok(Self::Png),
            "svg" => Ok(Self::Svg),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum VideoFileFormat {
    Mkv,
    Mp4,
    Webm,
}

impl FromStr for VideoFileFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        match s.as_str() {
            "mkv" => Ok(Self::Mkv),
            "mp4" => Ok(Self::Mp4),
            "webm" => Ok(Self::Webm),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AudioFileFormat {
    Mp3,
    Wav,
}

impl FromStr for AudioFileFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        match s.as_str() {
            "mp3" => Ok(Self::Mp3),
            "wav" => Ok(Self::Wav),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DocumentFileFormat {
    Doc,
    Docx,
    Json,
    Pdf,
    Toml,
    Txt,
    Yaml,
}

impl FromStr for DocumentFileFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        match s.as_str() {
            "doc" => Ok(Self::Doc),
            "docx" => Ok(Self::Docx),
            "json" => Ok(Self::Json),
            "pdf" => Ok(Self::Pdf),
            "toml" => Ok(Self::Toml),
            "txt" => Ok(Self::Txt),
            "yaml" => Ok(Self::Yaml),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CompressionFormat {
    Gzip,
    Zip,
}

impl FromStr for CompressionFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        match s.as_str() {
            "gz" => Ok(Self::Gzip),
            "zip" => Ok(Self::Zip),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FileFormat {
    Audio(AudioFileFormat),
    Compression(CompressionFormat),
    Document(DocumentFileFormat),
    Image(ImageFileFormat),
    Video(VideoFileFormat),
}

impl FromStr for FileFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        if let Ok(f) = AudioFileFormat::from_str(&s) {
            return Ok(FileFormat::Audio(f));
        }
        if let Ok(f) = CompressionFormat::from_str(&s) {
            return Ok(FileFormat::Compression(f));
        }
        if let Ok(f) = DocumentFileFormat::from_str(&s) {
            return Ok(FileFormat::Document(f));
        }
        if let Ok(f) = ImageFileFormat::from_str(&s) {
            return Ok(FileFormat::Image(f));
        }
        if let Ok(f) = VideoFileFormat::from_str(&s) {
            return Ok(FileFormat::Video(f));
        }

        Err(())
    }
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

// pub async fn ext_by_name(path: &str, file_name: &str) -> Result<String, AppErr> {
//     let mut dir = read_dir(path).await.map_err(|e| {
//         AppErr::new(
//             StatusCode::INTERNAL_SERVER_ERROR,
//             format!("error while read dir: {e}"),
//         )
//     })?;

//     while let Ok(Some(file)) = dir.next_entry().await {
//         let p = file.path().to_string_lossy().into_owned();
//         if p.contains(file_name) {
//             return Ok(file
//                 .path()
//                 .extension()
//                 .ok_or_else(|| {
//                     AppErr::new(StatusCode::INTERNAL_SERVER_ERROR, "error while read ext")
//                 })?
//                 .to_string_lossy()
//                 .into_owned());
//         }
//     }

//     Err(AppErr::new(StatusCode::INTERNAL_SERVER_ERROR, "no file"))
// }

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

pub async fn documents_convert(
    input_file_path: &str,
    output_file_path: &str,
) -> Result<(), AppErr> {
    Command::new("pandoc")
        .args(vec!["-f", &input_file_path, "-t", &output_file_path])
        .status()
        .await
        .map_err(|e| {
            AppErr::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("err while spawn pandoc task: {e}"),
            )
        })?;

    Ok(())
}

pub async fn rm_file(file_path: &str) -> Result<(), AppErr> {
    remove_file(file_path).await.map_err(|e| {
        AppErr::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("err while spawn rm task: {e}"),
        )
    })?;

    Ok(())
}

// pub async fn rm_dir(dir: &str) -> Result<(), AppErr> {
//     Command::new("rm")
//         .args(vec!["-r", dir])
//         .status()
//         .await
//         .map_err(|e| {
//             AppErr::new(
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 format!("err while spawn rm task: {e}"),
//             )
//         })?;

//     Ok(())
// }

pub fn can_convert(new_file_format: &str, file_extension: &str) -> Result<(), AppErr> {
    if new_file_format.to_lowercase() == file_extension.to_lowercase() {
        return Err(AppErr::new(
            StatusCode::BAD_REQUEST,
            "err: the same formats",
        ));
    }

    Ok(())
}

pub async fn convert(
    field: Field<'_>,
    file_name: &str,
    new_file_format: &str,
    app_data: &AppData,
) -> Result<String, AppErr> {
    let temp_folder = &app_data.temp_folder;

    let file_name_path = Path::new(&file_name);
    let file_extension = file_name_path.extension().unwrap().to_str().unwrap();

    can_convert(new_file_format, file_extension)?;

    let Ok(file_bytes) = field.bytes().await else {
        return Err(AppErr::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "no file bytes found",
        ));
    };

    let file_name_without_extension = file_name_path.file_stem().unwrap().to_str().unwrap();

    let rnd_string = gen_random_string();

    let download_folder = format!("{temp_folder}/{rnd_string}");

    let input_file_path = format!("{download_folder}/{file_name}");
    let output_file_path =
        format!("{download_folder}/{file_name_without_extension}.{new_file_format}");

    tokio::fs::create_dir(&download_folder).await.unwrap();
    let mut file = File::create(&input_file_path).await.unwrap();

    file.write_all(&file_bytes).await.unwrap();

    let file_format = FileFormat::from_str(new_file_format).unwrap();

    match file_format {
        FileFormat::Audio(_) | FileFormat::Image(_) | FileFormat::Video(_) => {
            ffmpeg_convert(&input_file_path, &output_file_path).await?;
        }
        FileFormat::Compression(f) => compress(&input_file_path, &output_file_path, f).await?,
        FileFormat::Document(_) => documents_convert(&input_file_path, &output_file_path).await?,
    }

    rm_file(&input_file_path).await.unwrap();

    Ok(output_file_path)
}

pub async fn compress(
    input_file_path: &str,
    output_file_path: &str,
    compression_format: CompressionFormat,
) -> Result<(), AppErr> {
    match compression_format {
        CompressionFormat::Gzip => {
            Command::new("gzip")
                .args(vec!["-9", &input_file_path])
                .status()
                .await
                .map_err(|e| {
                    AppErr::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("err while spawn zip task: {e}"),
                    )
                })?;
        }
        CompressionFormat::Zip => {
            Command::new("zip")
                .args(vec!["-qqj9", &output_file_path, &input_file_path])
                .status()
                .await
                .map_err(|e| {
                    AppErr::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("err while spawn zip task: {e}"),
                    )
                })?;
        }
    }

    Ok(())
}
