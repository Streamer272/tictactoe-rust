use std::fmt::{Display, Formatter, Result};
use terminal_size::terminal_size;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BoxContent {
    Empty,
    X,
    O,
}

#[derive(Clone, Copy)]
pub enum Turn {
    Player,
    Computer,
}

impl Display for BoxContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            BoxContent::Empty => write!(f, " "),
            BoxContent::X => write!(f, "X"),
            BoxContent::O => write!(f, "O"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Box {
    pub content: BoxContent,
}

pub enum Direction {
    Left,
    Down,
    Up,
    Right,
}

pub struct Field {
    boxes: [Box; 9],
    selected: i32,
}

impl Field {
    pub fn new() -> Field {
        Field {
            boxes: [Box {
                content: BoxContent::Empty,
            }; 9],
            selected: 4,
        }
    }

    pub fn move_selected(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                if self.selected - 1 >= 0 && self.selected % 3 != 0 {
                    self.selected -= 1;
                }
            }
            Direction::Down => {
                if self.selected + 3 < 9 {
                    self.selected += 3;
                }
            }
            Direction::Up => {
                if self.selected - 3 >= 0 {
                    self.selected -= 3;
                }
            }
            Direction::Right => {
                if self.selected + 1 < 9 && self.selected % 3 != 2 {
                    self.selected += 1;
                }
            }
        }
    }

    pub fn print(&self) {
        let (width, _) = terminal_size().unwrap();
        let mut output: String = String::from("");

        for row in 0..3 {
            let left_box = &self.boxes[row * 3];
            let middle_box = &self.boxes[row * 3 + 1];
            let right_box = &self.boxes[row * 3 + 2];

            for _ in 0..((width.0 - 13) / 2) {
                output.push_str(" ");
            }

            if self.selected == (row * 3) as i32 {
                output.push_str(
                    format!(
                        "[ {} ] {} | {} |",
                        left_box.content, middle_box.content, right_box.content
                    )
                    .as_str(),
                );
            } else if self.selected == (row * 3 + 1) as i32 {
                output.push_str(
                    format!(
                        "| {} [ {} ] {} |",
                        left_box.content, middle_box.content, right_box.content
                    )
                    .as_str(),
                );
            } else if self.selected == (row * 3 + 2) as i32 {
                output.push_str(
                    format!(
                        "| {} | {} [ {} ]",
                        left_box.content, middle_box.content, right_box.content
                    )
                    .as_str(),
                );
            } else {
                output.push_str(
                    format!(
                        "| {} | {} | {} |",
                        left_box.content, middle_box.content, right_box.content
                    )
                    .as_str(),
                );
            }

            // 13 is width if text
            for _ in 0..((width.0 - 13) / 2) {
                output.push_str(" ");
            }
        }

        print!("\r{}$ ", &output)
    }

    pub fn selected(&self) -> Box {
        return self.boxes[self.selected as usize];
    }

    pub fn flag(&mut self) {
        if self.selected().content != BoxContent::Empty {
            return;
        }

        self.selected().content = BoxContent::X;
    }
}
