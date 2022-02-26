use std::fmt::{Formatter, Display, Result};
use terminal_size::{terminal_size};

enum BoxContent {
    Empty,
    X,
    O,
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

pub struct Box {
    content: BoxContent,
    selected: bool,
}

impl Box {
    pub fn format() -> String {
        String::from("")
    }
}

pub struct Field {
    boxes: [Box; 9],
}

pub enum Direction {
    Left,
    Down,
    Up,
    Right,
}

impl Field {
    pub fn new() -> Field {
        Field {
            boxes: [
                Box{content: BoxContent::Empty, selected: false},
                Box{content: BoxContent::Empty, selected: false},
                Box{content: BoxContent::Empty, selected: false},
                Box{content: BoxContent::Empty, selected: false},
                Box{content: BoxContent::Empty, selected: true},
                Box{content: BoxContent::Empty, selected: false},
                Box{content: BoxContent::Empty, selected: false},
                Box{content: BoxContent::Empty, selected: false},
                Box{content: BoxContent::Empty, selected: false},
            ],
        }
    }

    pub fn get_box(&self, index: usize) -> &Box {
        &self.boxes[index]
    }

    pub fn move_selected(&mut self, direction: Direction) {
        let current = self.boxes.iter().position(|b| b.selected).unwrap();

        match direction {
            Direction::Left => {
                &self.boxes[current - 1].selected = true;
            },
            Direction::Down => {
                &self.boxes[current + 3].selected = true;
            },
            Direction::Up => {
                &self.boxes[current - 3].selected = true;
            },
            Direction::Right => {
                &self.boxes[current + 1].selected = true;
            },
        }
    }

    pub fn print(&self) {
        let (width, height) = terminal_size().unwrap();
        println!("term size: {}:{}", width.0, height.0);
        let mut output: String = String::from("");

        for row in 0..3 {
            let left_box = &self.boxes[row * 3];
            let middle_box = &self.boxes[row * 3 + 1];
            let right_box = &self.boxes[row * 3 + 2];

            for _ in 0..((width.0 - 13) / 2) {
                output.push_str(" ");
            }

            if left_box.selected {
                output.push_str(format!("> {} < {} | {} |", left_box.content, middle_box.content, right_box.content).as_str());
            }
            else if middle_box.selected {
                output.push_str(format!("| {} > {} < {} |", left_box.content, middle_box.content, right_box.content).as_str());
            }
            else if right_box.selected {
                output.push_str(format!("| {} | {} > {} <", left_box.content, middle_box.content, right_box.content).as_str());
            }
            else {
                output.push_str(format!("| {} | {} | {} |", left_box.content, middle_box.content, right_box.content).as_str());
            }

            for _ in 0..((width.0 - 13) / 2) {
                output.push_str(" ");
            }
        }

        print!("\r{}$ ", &output)
    }
}
