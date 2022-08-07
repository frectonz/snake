use macroquad::prelude::*;
use snake::{Board, CellType, Direction, Snake};

const BLOCK_SIZE: f32 = 30.;

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
    let rows: usize = (screen_height() / BLOCK_SIZE) as usize;
    let columns: usize = (screen_width() / BLOCK_SIZE) as usize;

    let mut board = Board::new(rows, columns);
    let mut snake = Snake::new();

    board.generate_food();

    let mut count = 0;
    loop {
        clear_background(WHITE);

        draw_board(&board);
        update_board(&mut board, &mut snake);
        next_frame().await;

        if count == 10 {
            count = 0;
            snake.update_movement(&mut board);
        }

        count += 1;
    }
}

fn update_board(board: &mut Board, snake: &mut Snake) {
    if !board.game_over() {
        if is_key_pressed(KeyCode::Up) && snake.direction() != &Direction::Down {
            snake.change_direction(Direction::Up);
        } else if is_key_pressed(KeyCode::Down) && snake.direction() != &Direction::Up {
            snake.change_direction(Direction::Down);
        } else if is_key_pressed(KeyCode::Left) && snake.direction() != &Direction::Right {
            snake.change_direction(Direction::Left);
        } else if is_key_pressed(KeyCode::Right) && snake.direction() != &Direction::Left {
            snake.change_direction(Direction::Right);
        }
        snake.update(board);
    }

    if is_key_pressed(KeyCode::Space) && board.game_over() {
        board.reset();
        snake.reset();
        board.generate_food();
    }
}

fn draw_board(board: &Board) {
    let width = board.columns() as f32 * BLOCK_SIZE;
    let height = board.rows() as f32 * BLOCK_SIZE;

    let offset_x = (screen_width() - width) / 2.;
    let offset_y = (screen_height() - height) / 2.;

    for c in board.cells().iter() {
        let x = BLOCK_SIZE * c.col() as f32 + offset_x;
        let y = BLOCK_SIZE * c.row() as f32 + offset_y;

        match c.cell_type() {
            CellType::Empty => draw_rectangle(x, y, BLOCK_SIZE, BLOCK_SIZE, BLACK),
            CellType::Snake => draw_rectangle(x, y, BLOCK_SIZE, BLOCK_SIZE, GREEN),
            CellType::Food => draw_rectangle(x, y, BLOCK_SIZE, BLOCK_SIZE, RED),
            CellType::SnakeHead => draw_rectangle(x, y, BLOCK_SIZE, BLOCK_SIZE, BLUE),
        };

        draw_rectangle_lines(x, y, BLOCK_SIZE, BLOCK_SIZE, 1., BLACK);
    }

    draw_rectangle_lines(offset_x, offset_y, width, height, 2., BLACK);

    if board.game_over() {
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
