use super::{Board, CellType};
use std::collections::LinkedList;

struct Part {
    col: usize,
    row: usize,
}

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

pub struct Snake {
    direction: Direction,
    parts: LinkedList<Part>,
}

impl Snake {
    pub fn new() -> Self {
        let mut parts = LinkedList::new();
        parts.push_back(Part { col: 5, row: 2 });
        parts.push_back(Part { col: 4, row: 2 });
        parts.push_back(Part { col: 3, row: 2 });
        parts.push_back(Part { col: 2, row: 2 });

        Self {
            parts,
            direction: Direction::Right,
        }
    }

    pub fn reset(&mut self) {
        self.parts.clear();
        self.parts.push_back(Part { col: 5, row: 2 });
        self.parts.push_back(Part { col: 4, row: 2 });
        self.parts.push_back(Part { col: 3, row: 2 });
        self.parts.push_back(Part { col: 2, row: 2 });
        self.direction = Direction::Right;
    }

    pub fn pop_end(&mut self, board: &mut Board) {
        match self.parts.pop_back() {
            Some(cell) => {
                board.set_cell(cell.col, cell.row, CellType::Empty);
            }
            None => {}
        };
    }

    pub fn move_up(&mut self, board: &mut Board) {
        let head = self.parts.front().unwrap();
        let new_head = Part {
            col: head.col,
            row: if head.row == 0 {
                board.rows() - 1
            } else {
                head.row - 1
            },
        };

        if board.is_snake(new_head.col, new_head.row) {
            board.end_game();
            return;
        }

        self.parts.push_front(new_head);
        self.pop_end(board);
    }

    pub fn move_down(&mut self, board: &mut Board) {
        let head = self.parts.front().unwrap();
        let new_head = Part {
            col: head.col,
            row: (head.row + 1) % board.rows(),
        };

        if board.is_snake(new_head.col, new_head.row) {
            board.end_game();
            return;
        }

        self.parts.push_front(new_head);
        self.pop_end(board);
    }

    pub fn move_left(&mut self, board: &mut Board) {
        let head = self.parts.front().unwrap();
        let new_head = Part {
            col: if head.col == 0 {
                board.columns() - 1
            } else {
                head.col - 1
            },
            row: head.row,
        };

        if board.is_snake(new_head.col, new_head.row) {
            board.end_game();
            return;
        }

        self.parts.push_front(new_head);
        self.pop_end(board);
    }

    pub fn move_right(&mut self, board: &mut Board) {
        let head = self.parts.front().unwrap();
        let new_head = Part {
            col: (head.col + 1) % board.columns(),
            row: head.row,
        };

        if board.is_snake(new_head.col, new_head.row) {
            board.end_game();
            return;
        }

        self.parts.push_front(new_head);
        self.pop_end(board);
    }

    pub fn grow(&mut self, board: &mut Board) {
        let tail = self.parts.back().unwrap();
        let new_tail = Part {
            col: tail.col,
            row: tail.row,
        };

        board.set_cell(new_tail.col, new_tail.row, CellType::Snake);
        self.parts.push_back(new_tail);
    }

    pub fn change_direction(&mut self, dir: Direction) {
        match self.direction {
            Direction::Up if dir != Direction::Down => {
                self.direction = dir;
            }
            Direction::Down if dir != Direction::Up => {
                self.direction = dir;
            }
            Direction::Left if dir != Direction::Right => {
                self.direction = dir;
            }
            Direction::Right if dir != Direction::Left => {
                self.direction = dir;
            }
            _ => {}
        };
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn update(&mut self, board: &mut Board) {
        let head = self.parts.front().unwrap();
        if board.is_food(head.col, head.row) {
            self.grow(board);
            board.generate_food();
        }

        for (i, c) in self.parts.iter().enumerate().rev() {
            if i == 0 {
                board.set_cell(c.col, c.row, CellType::SnakeHead);
            } else {
                board.set_cell(c.col, c.row, CellType::Snake);
            }
        }
    }

    pub fn update_movement(&mut self, board: &mut Board) {
        match self.direction {
            Direction::Up => self.move_up(board),
            Direction::Down => self.move_down(board),
            Direction::Left => self.move_left(board),
            Direction::Right => self.move_right(board),
        };
    }

    pub fn score(&self) -> usize {
        self.parts.len() - 4
    }
}
