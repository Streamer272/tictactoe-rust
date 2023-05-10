extern crate core;

mod field;

use field::{Field, Turn};
use std::io::{stdin, stdout, Write};
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let mut field = Field::new();
    let mut turn = Turn::Player;

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
            Key::Char('a') | Key::Char('h') | Key::Left => {
                field.move_selected(field::Direction::Left)
            }
            Key::Char('s') | Key::Char('j') | Key::Down => {
                field.move_selected(field::Direction::Down)
            }
            Key::Char('w') | Key::Char('k') | Key::Up => field.move_selected(field::Direction::Up),
            Key::Char('d') | Key::Char('l') | Key::Right => {
                field.move_selected(field::Direction::Right)
            }
            Key::Char(' ') | Key::Char('\n') => {
                if let Turn::Computer = turn {
                    continue;
                }
            }
            _ => (),
        }

        field.print();
        stdout.flush().unwrap();
    }
}
