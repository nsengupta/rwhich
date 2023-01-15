use std::{env, fs};
use std::collections::HashSet;
use std::os::unix::fs::MetadataExt;
use std::io;
use std::path::PathBuf;
use pico_args::Arguments;
use regex::Regex;

fn main() {
    println!("Hello, world!");

    let path_split_pattern = Regex::new(":");

    let all_args = Arguments::from_env();

    let mut files_to_search_for = all_args.free().unwrap();
    // println!("{:?}", all_args.free().unwrap());

    let path_from_env = env::var("PATH").unwrap();
    let env_path_components: Vec<&str> = path_from_env.split(":").collect();

    dbg!(&env_path_components);

    let mut all_chosen_paths: Vec<PathBuf> = Vec::new(); // of String, and not of &str
    for next_path in env_path_components {
        for next_dir_entry in fs::read_dir(next_path) {
            for next_file in next_dir_entry {

                let file_metadata = next_file.as_ref().unwrap().metadata().unwrap();
                if (!file_metadata.is_dir())
                    && file_metadata.mode() & 0o500 == 0o500 {
                    let chosen_path: PathBuf = next_file.as_ref().unwrap().path();
                    // let chosen_file: Option<&str> = chosen_path.to_str();
                    // Explicitly owning a String, made out of &str, below!
                    all_chosen_paths.push(chosen_path);
                }

            }
        }
    }

    // println!("{:?}", all_chosen_paths);

    let next_file_to_search_for = files_to_search_for.pop().unwrap();

    let mut final_list: Vec<PathBuf>  = Vec::new();

    for next_chosen_file in all_chosen_paths {
        let os_filename = next_chosen_file.file_name().unwrap();

        if os_filename.eq(next_file_to_search_for.as_str()) {
            println!("searching for {}, next one {}", next_file_to_search_for.as_str(), next_chosen_file.as_path().to_str().unwrap());
            final_list.push(PathBuf::from(next_chosen_file));
        }
    }

    let output: HashSet<PathBuf> = HashSet::from_iter(final_list.into_iter());

    println!("{:?}", output);

}
