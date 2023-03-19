use std::{fmt::Debug, sync::mpsc::SyncSender};

use rand::{thread_rng, Rng};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum CellType {
    Empty,
    Snake,
    Food,
    SnakeHead,
}

#[derive(Debug)]
pub struct Cell {
    col: usize,
    row: usize,
    cell: CellType,
}

impl Cell {
    pub fn col(&self) -> usize {
        self.col
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn cell_type(&self) -> CellType {
        self.cell
    }
}

pub enum GameEvent {
    FoodEaten,
    SnakeDied,
    SnakeChangedDirection,
}

pub struct Board {
    rows: usize,
    columns: usize,
    cells: Vec<Cell>,
    game_over: bool,
    paused: bool,
    event_sender: Option<SyncSender<GameEvent>>,
}

impl Board {
    pub fn new(rows: usize, columns: usize, event_sender: Option<SyncSender<GameEvent>>) -> Self {
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
            paused: false,
            event_sender,
        }
    }

    pub fn paused(&self) -> bool {
        self.paused
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn game_over(&self) -> bool {
        self.game_over
    }

    pub fn end_game(&mut self) {
        if let Some(event_sender) = &self.event_sender {
            let _ = event_sender.send(GameEvent::SnakeDied);
        }
        self.game_over = true;
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn set_cell(&mut self, col: usize, row: usize, cell: CellType) {
        let index = (col + row * self.columns) % self.cells.len();
        self.cells[index].cell = cell;
    }

    pub fn get_cell(&self, col: usize, row: usize) -> CellType {
        let index = (col + row * self.columns) % self.cells.len();
        self.cells[index].cell
    }

    pub fn is_food(&self, col: usize, row: usize) -> bool {
        self.get_cell(col, row) == CellType::Food
    }

    pub fn is_empty(&self, col: usize, row: usize) -> bool {
        self.get_cell(col, row) == CellType::Empty
    }

    pub fn is_snake(&self, col: usize, row: usize) -> bool {
        self.get_cell(col, row) == CellType::Snake
    }

    pub fn reset(&mut self) {
        for cell in &mut self.cells {
            cell.cell = CellType::Empty;
        }
        self.game_over = false;
        self.generate_food();
    }

    pub fn generate_food(&mut self) {
        let mut rng = thread_rng();
        let mut rand_row = rng.gen_range(0..self.rows);
        let mut rand_col = rng.gen_range(0..self.columns);

        while !self.is_empty(rand_col, rand_row) {
            rand_row = rng.gen_range(0..self.rows);
            rand_col = rng.gen_range(0..self.columns);
        }

        self.set_cell(rand_col, rand_row, CellType::Food);
    }

    pub(crate) fn food_eaten_event(&self) {
        if let Some(event_sender) = &self.event_sender {
            let _ = event_sender.send(GameEvent::FoodEaten);
        }
    }

    pub(crate) fn change_direction_event(&self) {
        if let Some(event_sender) = &self.event_sender {
            let _ = event_sender.send(GameEvent::SnakeChangedDirection);
        }
    }
}
