use std::path::{Path, PathBuf};

mod test;
mod utils;
mod cli;
mod commit;
mod config;
mod diff;
mod error;
mod files;
mod init;
mod remote;
mod repository;
mod storage;
mod version;
mod add;
mod ignore;
mod path;

fn main() {

    test_compo()


}

fn test_compo() {

    let igpath = PathBuf::from("./example/chak.ignore");
    let binding = std::env::current_dir().expect("");
    let nep = path::get_relative_path(&igpath, &*binding);
    println!("{:?}", nep);

}

fn test_ignore() {

        let gitignore_path = PathBuf::from("./example/chak.ignore");
    match ignore::parse_gitignore(&gitignore_path) {
        Ok(rules) => {
            println!("Global Ignore Patterns: {:#?}", rules.global_ignore);
            println!("Local Ignore Patterns: {:#?}", rules.local_ignore);
            println!("Exception Patterns: {:#?}", rules.exceptions);

            let test_path = PathBuf::from("node_modules/package-lock.json");
            if crate::ignore::is_ignored(&test_path, &rules) {
                println!("Path {:?} is ignored", test_path);
            } else {
                println!("Path {:?} is not ignored", test_path);
            }
        }
        Err(e) => {
            eprintln!("Error reading .gitignore: {}", e);
        }
    }

}