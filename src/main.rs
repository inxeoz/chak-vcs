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

fn main() {
    // Invoke CLI module to run the application
    chak_cli::run();
}
