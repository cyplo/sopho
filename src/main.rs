use anyhow::*;
use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use rexif::{ExifTag, TagValue};
use std::path::PathBuf;
use std::{fs, io};
use structopt::StructOpt;
use walkdir::WalkDir;

fn main() -> Result<()> {
    let options = CommandlineOptions::from_args();

    let path = fs::canonicalize(options.dir)?;
    println!("processing {:?}", path);
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.into_path())
        .filter(|e| e.is_file())
    {
        let from_exif = date_from_exif(entry.clone())?;
        if let Some(date) = from_exif {
            println!("{}: {}", entry.to_string_lossy(), date);
        }
        // get date from exif
        // get date from file name
        // if they don't agree - use exif
        // if there's no exif - use file name
    }
    Ok(())
}

fn date_from_exif(entry: PathBuf) -> Result<Option<NaiveDateTime>> {
    let exif =
        rexif::parse_file(entry.as_path()).context(format!("{}", entry.to_string_lossy()))?;
    let date = exif.entries.into_iter().find_map(|e| match e.tag {
        ExifTag::DateTime => Some(e),
        _ => None,
    });

    match date {
        None => Ok(None),
        Some(date) => Ok(Some(parse_date(date.value)?)),
    }
}

fn parse_date(value: TagValue) -> Result<NaiveDateTime> {
    match value {
        TagValue::Ascii(text) => {
            NaiveDateTime::parse_from_str(&text, "%Y:%m:%d %H:%M:%S").map_err(|e| anyhow!("{}", e))
        }
        _ => Err(anyhow!("")),
    }
}

#[derive(Clone, StructOpt, Debug)]
#[structopt(name = "sopho")]
pub struct CommandlineOptions {
    #[structopt(help = "Directory to process", index = 1)]
    pub dir: String,
}
