use chrono::{DateTime, Utc};
use clap::Parser;
use owo_colors::OwoColorize;
use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};
use strum_macros::Display;
use tabled::{
    Table, Tabled,
    settings::{Color, Style, object::Columns},
};

#[derive(Debug, Parser)]
#[command(version, about, long_about = "Coolest list directory ever")]
struct CLI {
    path: Option<PathBuf>,
}

fn main() {
    let cli = CLI::parse();

    let path = cli.path.unwrap_or(PathBuf::from("."));

    if let Ok(does_exists) = fs::exists(&path) {
        if does_exists {
            let files = get_files(&path);

            let mut table = Table::new(files);

            table.with(Style::rounded());

            table.modify(Columns::first(), Color::FG_BRIGHT_GREEN);
            table.modify(Columns::one(1), Color::FG_BRIGHT_CYAN);
            table.modify(Columns::one(2), Color::FG_BRIGHT_MAGENTA);
            table.modify(Columns::one(3), Color::FG_BRIGHT_YELLOW);

            println!("{}", table);
        } else {
            println!(
                "{}",
                format!("path '{}' does not exixts", path.display()).red()
            );
        }
    } else {
        println!(
            "{}",
            format!("error reading path '{}'", path.display()).red()
        );
    }
}
#[derive(Debug, Display)]
enum EntryType {
    File,
    Dir,
}
#[derive(Debug, Tabled)]
struct FileEntry {
    #[tabled{rename="Name"}]
    name: String,
    #[tabled{rename = "Type"}]
    e_type: EntryType,
    #[tabled{rename = "Size"}]
    len: u64,
    #[tabled{rename = "Modified"}]
    modified: String,
}

// get files

fn get_files(path: &Path) -> Vec<FileEntry> {
    let mut data = Vec::new();

    if let Ok(read_files) = fs::read_dir(path) {
        for entry in read_files {
            if let Ok(file) = entry {
                get_entry(file, &mut data);
            }
        }
    }

    data
}

fn get_entry(file: DirEntry, data: &mut Vec<FileEntry>) {
    if let Ok(meta) = fs::metadata(&file.path()) {
        data.push(FileEntry {
            name: file
                .file_name()
                .into_string()
                .unwrap_or("<unknown file name>".into()),
            e_type: if meta.is_file() {
                EntryType::File
            } else {
                EntryType::Dir
            },
            len: meta.len(),
            modified: if let Ok(modified) = meta.modified() {
                let date: DateTime<Utc> = modified.into();

                format!("{}", date.format("%Y-%m-%d %H:%M:%S"))
            } else {
                String::default()
            },
        });
    }
}
