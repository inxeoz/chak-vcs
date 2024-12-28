use clap::{App, Arg};

pub fn run() {
    let matches = App::new("Chak")
        .version("0.1")
        .about("Chak Version Control System")
        .arg(
            Arg::new("init")
                .long("init")
                .takes_value(false)
                .help("Initialize a new repository"),
        )
        .arg(
            Arg::new("commit")
                .long("commit")
                .takes_value(true)
                .help("Commit changes"),
        )
        .get_matches();

    if matches.is_present("init") {
        chak_repository::init();
    } else if let Some(message) = matches.value_of("commit") {
        chak_commit::commit(message);
    }
}
