pub struct Config {
    pub rows: Vec<String>,
}

impl Config {
    pub fn new() -> Config {
        use clap::Arg;
        const NAME: &str = env!("CARGO_PKG_NAME");
        let row_default = row_default();
        let matches = clap::App::new(NAME)
            .about("select a value in a vertical list (TUI)")
            .version(app_version().as_str())
            .arg(
                Arg::with_name("ROWS") // positional arguments
                    .required(true)
                    .multiple(true)
                    .empty_values(false)
                    .help("rows to display")
                    .default_value(row_default.as_str())
                    .hide_default_value(true),
            )
            .get_matches();

        let clap_rows = matches.values_of("ROWS").unwrap();

        Config {
            rows: clap_rows_to_vec(clap_rows),
        }
    }
}

/// Read from pipe or return an empty string.
fn row_default() -> String {
    use nix::unistd::isatty;
    use std::io::{self, Read};
    use std::os::unix::io::AsRawFd;

    let is_pipe = match isatty(io::stdin().as_raw_fd()) {
        Ok(false) => true,
        _ => false,
    };

    if is_pipe {
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_string(&mut buffer).unwrap(); // returns len
        buffer.trim().to_owned()
    } else {
        String::default()
    }
}

fn clap_rows_to_vec(arg_matches: clap::Values<'_>) -> Vec<String> {
    arg_matches
        .map(|rows| {
            rows.lines()
                .map(str::trim)
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .flatten()
        .filter(|s| !s.trim().is_empty())
        .collect()
}

fn app_version() -> String {
    use crate::const_fn::{GIT_CURRENT_HASH, RUSTC_VERSION};
    const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

    format!(
        "{}   rustc {}   git {}",
        APP_VERSION, RUSTC_VERSION, GIT_CURRENT_HASH
    )
}
