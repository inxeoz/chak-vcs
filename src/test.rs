
    use std::{env, fs};
    use std::path::{Path, PathBuf};
    use once_cell::sync::Lazy;


    static WORKING_DIR: Lazy<PathBuf> =
        Lazy::new(|| env::current_dir().expect("Failed to get current working directory"));

    static INTERNAL_WORKING_DIR_CHAK: Lazy<PathBuf> = Lazy::new(|| {
        env::current_dir()
            .expect("Failed to get current working directory")
            .join("../../.chak")
    });
