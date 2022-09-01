use macroquad::prelude::*;
use snake::{Board, CellType, Direction, Snake};

const BLOCK_SIZE: f32 = 30.;

fn window_conf() -> Conf {
    Conf {
        window_title: "Snake".to_owned(),
        window_width: 800,
        window_height: 800,
        fullscreen: true,
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
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        draw_board(&board, &snake);
        update_board(&mut board, &mut snake);
        next_frame().await;

        if count == 5 {
            count = 0;
            snake.update_movement(&mut board);
        }

        count += 1;
    }
}

fn update_board(board: &mut Board, snake: &mut Snake) {
    if !board.game_over() {
        if is_key_pressed(KeyCode::Up) {
            snake.change_direction(Direction::Up);
        } else if is_key_pressed(KeyCode::Down) {
            snake.change_direction(Direction::Down);
        } else if is_key_pressed(KeyCode::Left) {
            snake.change_direction(Direction::Left);
        } else if is_key_pressed(KeyCode::Right) {
            snake.change_direction(Direction::Right);
        }
        snake.update(board);
    }

    if is_key_pressed(KeyCode::Space) && board.game_over() {
        board.reset();
        snake.reset();
    }
}

fn draw_board(board: &Board, snake: &Snake) {
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

        draw_rectangle_lines(x, y, BLOCK_SIZE, BLOCK_SIZE, 1., WHITE);
    }

    draw_rectangle_lines(offset_x, offset_y, width, height, 2., BLACK);

    let start_pos = height + offset_y;
    draw_text(
        &format!("Score: {}", snake.score()),
        offset_x,
        start_pos + 30.,
        30.,
        WHITE,
    );
    draw_text(
        &format!("FPS: {}", get_fps()),
        offset_x,
        start_pos + 60.,
        30.,
        WHITE,
    );
    draw_text(
        &format!("{} seconds", get_time().floor()),
        offset_x,
        start_pos + 90.,
        30.,
        WHITE,
    );

    if board.game_over() {
        draw_centered_text("GAME OVER", screen_width() / 2., offset_y - 80., 100., RED);
        draw_centered_text(
            "Press <space> to restart the game",
            screen_width() / 2.,
            offset_y - 40.,
            30.,
            WHITE,
        );
        draw_centered_text(
            "Press <q> to quit the game",
            screen_width() / 2.,
            offset_y - 20.,
            30.,
            WHITE,
        );
        return;
    } else {
        draw_centered_text("SNAKE", screen_width() / 2., offset_y - 80., 100., GREEN);
        draw_centered_text(
            "Press <q> to quit the game",
            screen_width() / 2.,
            offset_y - 40.,
            30.,
            WHITE,
        );
        draw_centered_text(
            "Use arrow keys for movement",
            screen_width() / 2.,
            offset_y - 20.,
            30.,
            WHITE,
        );
    }
}

fn draw_centered_text(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    let dimensions = measure_text(text, None, font_size as u16, 1.0);
    draw_text(text, x - dimensions.width / 2., y, font_size, color);
}
