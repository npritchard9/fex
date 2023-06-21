use anyhow::anyhow;
use colored::Colorize;
use std::{
    fmt::Display,
    fs::{read_dir, DirEntry},
    time::UNIX_EPOCH,
};

use chrono::{Datelike, Month, TimeZone, Timelike};

enum FileType {
    File,
    Dir,
    HiddenFile,
    HiddenDir,
}

struct FileInfo {
    name: String,
    date: String,
    time: String,
    len: u64,
    file_type: FileType,
}

impl Display for FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use FileType::*;
        let name = match self.file_type {
            File | HiddenFile => self.name.white(),
            Dir | HiddenDir => self.name.blue(),
        };
        writeln!(
            f,
            "{} {} {} {}",
            name,
            self.date.green(),
            self.time.green(),
            self.len.to_string().blue()
        )
    }
}

impl From<DirEntry> for FileInfo {
    fn from(entry: DirEntry) -> Self {
        if let Ok(meta) = entry.metadata() {
            let secs = meta
                .modified()
                .expect("to be able to read modified")
                .duration_since(UNIX_EPOCH)
                .expect("to be able to read duration since")
                .as_secs();
            let date = chrono::Utc.timestamp_opt(secs as i64, 0).unwrap();
            let month = Month::try_from(u8::try_from(date.month()).unwrap())
                .ok()
                .unwrap();
            let day = date.day();
            let hours = date.hour();
            let mins = date.minute();
            let time = format!("{}:{}", hours, mins);
            let date = format!("{} {}", day, month.name());
            let name = entry
                .file_name()
                .to_str()
                .expect("to be able to read the name")
                .to_string();
            let file_type = if name.starts_with(".") {
                if meta.is_file() {
                    FileType::HiddenFile
                } else {
                    FileType::HiddenDir
                }
            } else {
                if meta.is_file() {
                    FileType::File
                } else {
                    FileType::Dir
                }
            };
            FileInfo {
                name,
                date,
                time,
                len: entry.metadata().unwrap().len(),
                file_type,
            }
        } else {
            FileInfo {
                name: String::from(""),
                date: String::from(""),
                time: String::from(""),
                len: 0,
                file_type: FileType::File,
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let entries = read_dir(".")?
        .map(|e| FileInfo::from(e.unwrap()))
        .collect::<Vec<_>>();
    for e in entries {
        println!("{}", e);
    }
    anyhow::Ok(())
}
