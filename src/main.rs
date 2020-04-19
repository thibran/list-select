mod config;
mod const_fn;
mod tui;

use crate::config::Config;
use crate::tui::tui;

fn main() {
    let cfg = Config::new();

    match cfg.rows.len() {
        1 => println!("{}", cfg.rows[0]),
        _ => println!("{}", tui(cfg)),
    }
}
