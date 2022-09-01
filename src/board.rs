use macroquad::rand;

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

#[derive(Debug)]
pub struct Board {
    rows: usize,
    columns: usize,
    cells: Vec<Cell>,
    game_over: bool,
}

impl Board {
    pub fn new(rows: usize, columns: usize) -> Self {
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

    pub fn game_over(&self) -> bool {
        self.game_over
    }

    pub fn end_game(&mut self) {
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
        let mut rand_row = rand::gen_range(0, self.rows);
        let mut rand_col = rand::gen_range(0, self.columns);

        while !self.is_empty(rand_col, rand_row) {
            rand_row = rand::gen_range(0, self.rows);
            rand_col = rand::gen_range(0, self.columns);
        }

        self.set_cell(rand_col, rand_row, CellType::Food);
    }
}
