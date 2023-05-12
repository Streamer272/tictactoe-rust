extern crate core;

mod field;

use field::{BoxContent, Direction, Field, Turn};
use std::io::{stdin, stdout, Write};
use std::process::exit;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let mut field = Field::new();

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    field.print();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();
        stdout.flush().unwrap();

        match c.unwrap() {
            Key::Char('q') | Key::Ctrl('c') | Key::End | Key::Esc => {
                stdout.flush().unwrap();
                exit(0);
            }
            Key::Char('a') | Key::Char('h') | Key::Left => field.move_selected(Direction::Left),
            Key::Char('s') | Key::Char('j') | Key::Down => field.move_selected(Direction::Down),
            Key::Char('w') | Key::Char('k') | Key::Up => field.move_selected(Direction::Up),
            Key::Char('d') | Key::Char('l') | Key::Right => field.move_selected(Direction::Right),
            Key::Char(' ') | Key::Char('\n') => field.flag(),
            _ => (),
        }

        field.print();
        stdout.flush().unwrap();
    }
}
