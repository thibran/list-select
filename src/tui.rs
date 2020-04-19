use crate::config::Config;
use tuikit::attr::{Attr, Color, Effect};
use tuikit::term::Term;

const ATTR_MARKED: Attr = Attr {
    fg: Color::Default,
    bg: Color::Rgb(37, 40, 41), // #5F676A
    effect: Effect::empty(),
};

pub fn tui(cfg: Config) -> String {
    use std::cmp::{max, min};
    use std::process;
    use tuikit::prelude::*;

    let col = 3;
    let mut row = 0;

    let rows_len = cfg.rows.len();
    let term = Term::with_height(TermHeight::Fixed(rows_len)).unwrap();
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
                return cfg.rows[row].to_owned();
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
        draw(&term, col, row, &cfg.rows);
        let _ = term.present();
    }

    unreachable!()
}

fn draw(term: &Term, col: usize, row: usize, rows: &[String]) {
    rows.iter().enumerate().for_each(|(i, v)| {
        if i == row {
            let _ = term.print(i, 0, ">");
            let _ = term.print_with_attr(i, col, v, ATTR_MARKED);
        } else {
            let _ = term.print(i, col, v);
        }
    })
}
