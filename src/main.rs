mod const_fn;

use std::process;

fn main() {
    let cfg = args_to_config();

    match cfg.len() {
        1 => println!("{}", cfg[0]),
        _ => {
            let s = tui(cfg);
            println!("{}", s);
        }
    }
}

fn args_to_config() -> Vec<String> {
    use clap::Arg;
    use nix::unistd::isatty;
    use std::io::{self, Read};
    use std::os::unix::io::AsRawFd;
    const NAME: &str = env!("CARGO_PKG_NAME");

    let is_pipe = match isatty(io::stdin().as_raw_fd()) {
        Ok(false) => true,
        _ => false,
    };

    let row_default = if is_pipe {
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_string(&mut buffer).unwrap(); // returns len
        buffer.trim().to_owned()
    } else {
        String::default()
    };

    let matches = clap::App::new(NAME)
        .about("select a value in a vertical list (TUI)")
        .version(app_version().as_str())
        .arg(
            Arg::with_name("ROWS") // positional arguments
                .required(true)
                .multiple(true)
                .empty_values(false)
                .help("rows to display")
                .default_value(&row_default)
                .hide_default_value(true),
        )
        .get_matches();

    return matches
        .values_of("ROWS")
        .unwrap()
        .map(|rows| {
            rows.lines()
                .map(str::trim)
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .flatten()
        .filter(|s| !s.trim().is_empty())
        .collect();

    fn app_version() -> String {
        use crate::const_fn::{GIT_CURRENT_HASH, RUSTC_VERSION};

        const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
        format!(
            "{}   rustc {}   git {}",
            APP_VERSION, RUSTC_VERSION, GIT_CURRENT_HASH
        )
    }
}

fn tui(lines: Vec<String>) -> String {
    use std::cmp::{max, min};
    use tuikit::attr::{Attr, Color, Effect};
    use tuikit::prelude::*;

    let col = 3;
    let mut row = 0;

    let attr_marked = Attr {
        fg: Color::Default,
        bg: Color::Rgb(37, 40, 41), // #5f676a
        effect: Effect::empty(),
    };

    let lines_len = lines.len();
    let term = Term::with_height(TermHeight::Fixed(lines_len)).unwrap();
    let _ = term.present(); // sync internal buffer with terminal

    while let Ok(ev) = term.poll_event() {
        let _ = term.clear();

        let (_width, height) = term.term_size().unwrap();
        match ev {
            Event::Key(Key::ESC)
            | Event::Key(Key::Char('q'))
            | Event::Key(Key::Ctrl('c'))
            | Event::Key(Key::Ctrl('d')) => {
                // make sure clean-up routines are called
                drop(term);
                process::exit(2);
            }
            Event::Key(Key::Enter) | Event::Key(Key::Tab) => {
                return lines.get(row).unwrap().to_owned()
            }
            Event::Key(Key::Up)
            | Event::Key(Key::Ctrl('p'))
            | Event::Key(Key::Char('k')) => {
                row = if row != 0 { max(row - 1, 0) } else { 0 }
            }
            Event::Key(Key::Down)
            | Event::Key(Key::Ctrl('n'))
            | Event::Key(Key::Char('j')) => row = min(row + 1, height - 1),
            _ => {}
        }
        lines.iter().enumerate().for_each(|(i, v)| {
            if i == row {
                let _ = term.print(i, 0, ">");
                let _ = term.print_with_attr(i, col, v, attr_marked);
            } else {
                let _ = term.print(i, col, v);
            }
        });
        let _ = term.present();
    }

    unreachable!()
}
