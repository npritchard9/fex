use std::{fs::read_dir, time::UNIX_EPOCH};

use chrono::{Datelike, Month, TimeZone};

fn main() -> anyhow::Result<()> {
    let entries = read_dir(".")?.map(|e| e.unwrap()).collect::<Vec<_>>();
    for entry in entries {
        let secs = entry
            .metadata()?
            .modified()?
            .duration_since(UNIX_EPOCH)?
            .as_secs();
        let date = chrono::Utc.timestamp_opt(secs as i64, 0).unwrap();
        let year = date.year();
        let month = Month::try_from(u8::try_from(date.month()).unwrap())
            .ok()
            .unwrap();
        let day = date.day();
        let full_date = format!("{} {}, {}", month.name(), day, year);
        println!(
            "{}, {}, {}",
            entry.file_name().to_str().unwrap(),
            full_date,
            entry.metadata()?.len()
        )
    }
    anyhow::Ok(())
}
