mod cli;
mod commit;
mod config;
mod diff;
mod error;
mod remote;
mod repository;
mod storage;
mod utils;
mod version;
mod add;
mod init;
mod test;

fn main() {

    let file_path = std::path::Path::new("./example/test.txt");

    let chak_folder = std::path::Path::new("./chak/");
    init::init_vcs(chak_folder).expect("TODO: panic message");

    let blob_path = std::path::Path::new("./chak/blobs");
    let hash_str = add::create_blob(&file_path, &blob_path).expect("Failed to create blob");

    println!("{}", hash_str);


    test::test_create_dir_snapshot()

}
