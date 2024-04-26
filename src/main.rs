#![allow(unused)]

mod args;

use std::{error::Error, fs::metadata, path::PathBuf};

use file_size::fit_4;
use walkdir::{DirEntry, Error as WalkdirError, WalkDir};

const DIR: &str = "./";
const TOP_NUMS: usize = 5;

fn main() {
    match exec() {
        Ok(_) => (),
        Err(error) => {
            println!("Error: {}", error)
        }
    }
}

struct Entry {
    path: PathBuf,
    size: u64,
}

fn exec() -> Result<(), Box<dyn Error>> {
    let mut total_file_number: u32 = 0;
    let mut total_folder_number: u32 = 0;
    let mut total_size: u64 = 0;
    let mut tops: Vec<Entry> = Vec::with_capacity(TOP_NUMS + 1);
    let mut min_of_tops: u64 = 0;

    for entry in WalkDir::new(DIR)
        .into_iter()
        .filter_map(|e: Result<DirEntry, WalkdirError>| e.ok())
    {
        let path: &std::path::Path = entry.path();

        if (path.is_file() && !path.is_symlink()) {
            total_file_number += 1;
            let size = entry.metadata()?.len();
            total_size += size;

            if min_of_tops < size {
                tops.push(Entry {
                    path: path.to_path_buf(),
                    size,
                });
                tops.sort_by(|a, b| a.size.cmp(&b.size));
                if (tops.len() > TOP_NUMS) {
                    tops.pop();
                }
            }

            min_of_tops = tops.last().map(|f| f.size).unwrap_or(0)

        }

        if (path.is_dir() && !path.is_symlink()) {
            total_folder_number += 1;
        }
    }



    println!("the total number of files are: {}", total_file_number);
    println!("the total number of folders are: {}", total_folder_number);
    println!("the total file size are: {}", fit_4(total_size));

    println!("top {} largest files are", tops.len());
    for Entry {size, path} in tops.iter() {
        println!("{:<4} - {:?}", fit_4(*size), path.to_string_lossy())
    }

    Ok(())
}
