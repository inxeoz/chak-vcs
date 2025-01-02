
    use std::{env, fs};
    use std::collections::{HashMap, HashSet};
    use std::fs::File;
    use std::path::{Path, PathBuf};
    use once_cell::sync::Lazy;
    use tempfile::TempDir;
    use crate::init;
    use crate::add::CHAKRA;

    use crate::files;

    static WORKING_DIR: Lazy<PathBuf> =
        Lazy::new(|| env::current_dir().expect("Failed to get current working directory"));

    static INTERNAL_WORKING_DIR_CHAK: Lazy<PathBuf> = Lazy::new(|| {
        env::current_dir()
            .expect("Failed to get current working directory")
            .join("../../.chak")
    });


    pub fn test_local_ignore_file_to_set() {
        let ignore_file_path = PathBuf::from("../../chak.ignore");
        let working_root_dir = &WORKING_DIR;
        let set = CHAKRA::local_ignore_file_to_set(&ignore_file_path, working_root_dir);
        println!("----> {:?}", set);
    }


    fn test_global_ignore_file_to_set() {
        let ignore_file_path = INTERNAL_WORKING_DIR_CHAK.join("../../chak.ignore");
        let working_dir = &WORKING_DIR;
        let set = CHAKRA::global_ignore_file_to_set(&ignore_file_path, working_dir);
        println!("----> {:?}", set);

        //write to file
        let sorted_string_vec = files::sort_by_component_count(&set);
        files::write_to_file_with_vec(&ignore_file_path, sorted_string_vec, false);
    }

    fn test_init_vcs() {
        init::init_vcs(&INTERNAL_WORKING_DIR_CHAK).expect("cant init");
    }

    fn test_create_dir_snapshot_empty_dir() {
        let temp_dir = TempDir::new().unwrap();
        CHAKRA::create_dir_snapshot(temp_dir.path());
        // No errors or output expected
    }


    fn test_create_dir_snapshot_with_files() {
        let temp_dir = TempDir::new().unwrap();
        File::create(temp_dir.path().join("file1.txt")).unwrap();
        File::create(temp_dir.path().join("file2.txt")).unwrap();
        CHAKRA::create_dir_snapshot(temp_dir.path());
        // Files should be logged
    }


    fn test_resolve_path() {
        let working_root = Path::new("/home/user/project");
        assert_eq!(
            CHAKRA::resolve_path(Path::new("../subdir"), working_root),
            PathBuf::from("/home/user/subdir")
        );
        assert_eq!(
            CHAKRA::resolve_path(Path::new("./file.txt"), working_root),
            PathBuf::from("/home/user/project/file.txt")
        );
    }

    fn test_folder_entity_to_set() {
        let temp_dir = TempDir::new().unwrap();
        File::create(temp_dir.path().join("file1.txt")).unwrap();
        File::create(temp_dir.path().join("file2.txt")).unwrap();
        let entities = CHAKRA::folder_entity_to_set(temp_dir.path());
        assert_eq!(entities.len(), 2);
    }


    fn test_create_blob() {
        let temp_dir = TempDir::new().unwrap();
        let blob_dir = temp_dir.path().join("blobs");
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, b"test content").unwrap();

        let hash = CHAKRA::create_blob(&file_path, &blob_dir).unwrap();
        assert!(blob_dir.join(&hash[0..2]).join(&hash[2..]).exists());
    }

    fn test_snapshot_directory() {
        let temp_dir = TempDir::new().unwrap();
        let blob_dir = temp_dir.path().join("blobs");
        fs::create_dir(&blob_dir).unwrap();

        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, b"content").unwrap();

        let snapshot = CHAKRA::snapshot_directory(temp_dir.path(), &blob_dir).unwrap();
        assert_eq!(snapshot.len(), 1);
    }

    fn test_diff_snapshots() {
        let mut old_snapshot = HashMap::new();
        old_snapshot.insert(PathBuf::from("file1.txt"), "hash1".to_string());

        let mut new_snapshot = HashMap::new();
        new_snapshot.insert(PathBuf::from("file1.txt"), "hash2".to_string());
        new_snapshot.insert(PathBuf::from("file2.txt"), "hash3".to_string());

        let changes = CHAKRA::diff_snapshots(&old_snapshot, &new_snapshot);
        assert_eq!(changes.len(), 2);
    }


    fn test_compute_dir_hash() {
        let mut snapshot = HashMap::new();
        snapshot.insert(PathBuf::from("file1.txt"), "hash1".to_string());
        snapshot.insert(PathBuf::from("file2.txt"), "hash2".to_string());

        let hash = CHAKRA::compute_dir_hash(&snapshot);
        assert!(!hash.is_empty());
    }

    fn test_init_repository() {
        let temp_dir = TempDir::new().unwrap();
        CHAKRA::init_repository(temp_dir.path()).unwrap();

        let chakra_dir = temp_dir.path().join(".chakra");
        assert!(chakra_dir.exists());
        assert!(chakra_dir.join("blobs").exists());
        assert!(chakra_dir.join("config").exists());
    }


    fn test_filter_with_ignore() {
        let mut snapshot = HashMap::new();
        snapshot.insert(PathBuf::from("file1.txt"), "hash1".to_string());
        snapshot.insert(PathBuf::from("file2.txt"), "hash2".to_string());

        let mut ignore_rules = HashSet::new();
        ignore_rules.insert(PathBuf::from("file1.txt"));

        let filtered = CHAKRA::filter_with_ignore(snapshot, &ignore_rules);
        assert_eq!(filtered.len(), 1);
        assert!(filtered.contains_key(&PathBuf::from("file2.txt")));
    }

