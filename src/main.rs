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

    use std::env;

    fn main() {
        match env::current_dir() {
            Ok(path) => println!("Current directory: {:?}", path),
            Err(e) => println!("Error getting current directory: {}", e),
        }
    }

    main();
 println!("");
    // test::test_create_dir_snapshot()
  //  test::test_read_ignore();
   // test::test_folder_entity_to_set()
   //test::test_ignore_file_to_set()

   // test::test_print_compo_path()

    test::test_parse_ignore_file();



}
