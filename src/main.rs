use std::collections::LinkedList;

use macroquad::prelude::*;

const BLOCK_SIZE: f32 = 50.;

#[derive(Copy, Clone, PartialEq, Debug)]
enum CellType {
    Empty,
    Snake,
    Food,
    SnakeHead,
}

#[derive(Debug)]
struct Cell {
    col: usize,
    row: usize,
    cell: CellType,
}

#[derive(Debug)]
struct Board {
    rows: usize,
    columns: usize,
    cells: Vec<Cell>,
    game_over: bool,
}

impl Board {
    fn new() -> Self {
        let rows: usize = (screen_height() / BLOCK_SIZE) as usize;
        let columns: usize = (screen_width() / BLOCK_SIZE) as usize;

        let mut cells = Vec::with_capacity(rows * columns);

        for row in 0..rows {
            for col in 0..columns {
                cells.push(Cell {
                    col,
                    row,
                    cell: CellType::Empty,
                });
            }
        }

        Self {
            rows,
            columns,
            cells,
            game_over: false,
        }
    }

    fn set_cell(&mut self, col: usize, row: usize, cell: CellType) {
        let index = (col + row * self.columns) % self.cells.len();
        self.cells[index].cell = cell;
    }

    fn get_cell(&self, col: usize, row: usize) -> CellType {
        let index = (col + row * self.columns) % self.cells.len();
        self.cells[index].cell
    }

    fn is_food(&self, col: usize, row: usize) -> bool {
        self.get_cell(col, row) == CellType::Food
    }

    fn is_empty(&self, col: usize, row: usize) -> bool {
        self.get_cell(col, row) == CellType::Empty
    }

    fn is_snake(&self, col: usize, row: usize) -> bool {
        self.get_cell(col, row) == CellType::Snake
    }

    fn generate_food(&mut self) {
        let mut rand_row = rand::gen_range(0, self.rows);
        let mut rand_col = rand::gen_range(0, self.columns);

        while !self.is_empty(rand_col, rand_row) {
            rand_row = rand::gen_range(0, self.rows);
            rand_col = rand::gen_range(0, self.columns);
        }

        self.set_cell(rand_col, rand_row, CellType::Food);
    }

    fn draw(&self) {
        let width = self.columns as f32 * BLOCK_SIZE;
        let height = self.rows as f32 * BLOCK_SIZE;

        let offset_x = (screen_width() - width) / 2.;
        let offset_y = (screen_height() - height) / 2.;

        for c in self.cells.iter() {
            let x = BLOCK_SIZE * c.col as f32 + offset_x;
            let y = BLOCK_SIZE * c.row as f32 + offset_y;

            match c.cell {
                CellType::Empty => {
                    draw_rectangle(x, y, BLOCK_SIZE, BLOCK_SIZE, GRAY);
                }
                CellType::Snake => {
                    draw_rectangle(x, y, BLOCK_SIZE, BLOCK_SIZE, GREEN);
                }
                CellType::Food => {
                    draw_rectangle(x, y, BLOCK_SIZE, BLOCK_SIZE, RED);
                }
                CellType::SnakeHead => {
                    draw_rectangle(x, y, BLOCK_SIZE, BLOCK_SIZE, BLUE);
                }
            }

            draw_rectangle_lines(x, y, BLOCK_SIZE, BLOCK_SIZE, 1., BLACK);

            draw_text(
                &format!("({},{})", c.col, c.row),
                x + 2.,
                y + 15.,
                15.,
                WHITE,
            )
        }

        draw_rectangle_lines(offset_x, offset_y, width, height, 2., BLACK);

        if self.game_over {
            draw_text(
                "GAME OVER",
                offset_x + width / 2. - (100.),
                offset_y + height / 2.,
                50.,
                WHITE,
            );
            return;
        }
    }
}

struct Part {
    col: usize,
    row: usize,
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

struct Snake {
    direction: Direction,
    parts: LinkedList<Part>,
}

impl Snake {
    fn new() -> Self {
        let mut parts = LinkedList::new();
        parts.push_back(Part { col: 2, row: 2 });

        Self {
            parts,
            direction: Direction::Right,
        }
    }

    fn pop_end(&mut self, board: &mut Board) {
        match self.parts.pop_back() {
            Some(cell) => {
                board.set_cell(cell.col, cell.row, CellType::Empty);
            }
            None => {}
        };
    }

    fn move_up(&mut self, board: &mut Board) {
        let head = self.parts.front().unwrap();
        let new_head = Part {
            col: head.col,
            row: if head.row == 0 {
                board.rows - 1
            } else {
                head.row - 1
            },
        };

        if board.is_snake(new_head.col, new_head.row) {
            board.game_over = true;
            return;
        }

        self.parts.push_front(new_head);
        self.pop_end(board);
    }

    fn move_down(&mut self, board: &mut Board) {
        let head = self.parts.front().unwrap();
        let new_head = Part {
            col: head.col,
            row: (head.row + 1) % board.rows,
        };

        if board.is_snake(new_head.col, new_head.row) {
            board.game_over = true;
            return;
        }

        self.parts.push_front(new_head);
        self.pop_end(board);
    }

    fn move_left(&mut self, board: &mut Board) {
        let head = self.parts.front().unwrap();
        let new_head = Part {
            col: if head.col == 0 {
                board.columns - 1
            } else {
                head.col - 1
            },
            row: head.row,
        };

        if board.is_snake(new_head.col, new_head.row) {
            board.game_over = true;
            return;
        }

        self.parts.push_front(new_head);
        self.pop_end(board);
    }

    fn move_right(&mut self, board: &mut Board) {
        let head = self.parts.front().unwrap();
        let new_head = Part {
            col: (head.col + 1) % board.columns,
            row: head.row,
        };

        if board.is_snake(new_head.col, new_head.row) {
            board.game_over = true;
            return;
        }

        self.parts.push_front(new_head);
        self.pop_end(board);
    }

    fn grow(&mut self, board: &mut Board) {
        let tail = self.parts.back().unwrap();
        let new_tail = Part {
            col: tail.col,
            row: tail.row,
        };

        board.set_cell(new_tail.col, new_tail.row, CellType::Snake);
        self.parts.push_back(new_tail);
    }

    fn update(&mut self, board: &mut Board) {
        if is_key_pressed(KeyCode::Up) && self.direction != Direction::Down {
            self.direction = Direction::Up;
        } else if is_key_pressed(KeyCode::Down) && self.direction != Direction::Up {
            self.direction = Direction::Down;
        } else if is_key_pressed(KeyCode::Left) && self.direction != Direction::Right {
            self.direction = Direction::Left;
        } else if is_key_pressed(KeyCode::Right) && self.direction != Direction::Left {
            self.direction = Direction::Right;
        }

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

    fn update_movement(&mut self, board: &mut Board) {
        match self.direction {
            Direction::Up => self.move_up(board),
            Direction::Down => self.move_down(board),
            Direction::Left => self.move_left(board),
            Direction::Right => self.move_right(board),
        };
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Snake".to_owned(),
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut board = Board::new();
    let mut snake = Snake::new();

    board.generate_food();

    let mut counter = 0;

    loop {
        clear_background(WHITE);

        board.draw();

        if !board.game_over {
            snake.update(&mut board);
        }

        if counter % 10 == 0 {
            snake.update_movement(&mut board)
        }

        counter += 1;
        next_frame().await
    }
}
