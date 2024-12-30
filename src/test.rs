use std::path::Path;
use crate::add;
use crate::init;

pub fn test_init() {

    let chak_folder = std::path::Path::new("./chak/");
    init::init_vcs(chak_folder).expect("TODO: panic message");

}
pub fn test_create_dir_snapshot() {

    let root_dir = Path::new("./example/");
    add::create_dir_snapshot(root_dir)
}

pub fn test_read_ignore() {
    let ignore_path = Path::new("./example/fold2/.ignore");
    let global_ignore_path = Path::new("./chak/chak.ignore");
   let ignore =  add::parse_ignore_file(ignore_path, global_ignore_path);
    println!("{:?}", ignore);
}


pub fn test_add_blob() {
    let file_path = std::path::Path::new("./example/test.txt");
    let blob_path = std::path::Path::new("./chak/blobs");
    let hash_str = add::create_blob(&file_path, &blob_path).expect("Failed to create blob");
    println!("{}", hash_str);
}

pub fn test_folder_entity_to_set () {

    let folder_path = Path::new("./example/");
    add::folder_entity_to_set(folder_path);
}

pub fn test_local_ignore_file_to_set() {
    let ignore_file_path =  Path::new("./example/fold2/.ignore");
    let ignore = add::local_ignore_file_to_set(ignore_file_path);
    println!("{:?}", ignore);
}

pub fn test_print_compo_path() {
    let path = Path::new("./example/fold2/nested_fold/../../fold1/f2.txt");

   // add::print_compo_path(path);
    add::resolve_path(path);
}


pub fn test_parse_ignore_file() {
    let local_ignore_path = Path::new("./example/fold2/.ignore");
    let global_ignore_path = Path::new("./chak/chak.ignore");
    let ignore = add::parse_ignore_file(local_ignore_path, global_ignore_path);

    println!("---> \n{:?}", ignore);
}

pub fn test_global_ignore_file_to_set() {

    let ignore_file_path =  Path::new("./chak/chak.ignore");
    let ignore = add::global_ignore_file_to_set(ignore_file_path);
    println!("{:?}", ignore);

}