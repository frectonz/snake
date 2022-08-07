use ruscii::{
    app::{App, Config, State},
    drawing::Pencil,
    keyboard::{Key, KeyEvent},
    spatial::Vec2,
    terminal::{Color, Window},
};
use snake::{Board, CellType, Direction, Snake};

fn main() {
    let config = Config::new().fps(15);

    let mut app = App::config(config);

    let size = app.window().size();
    let mut board = Board::new(size.y.try_into().unwrap(), size.x.try_into().unwrap());
    let mut snake = Snake::new();

    board.generate_food();

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Up) => {
                    if snake.direction() != &Direction::Down {
                        snake.change_direction(Direction::Up);
                    }
                }
                KeyEvent::Pressed(Key::Down) => {
                    if snake.direction() != &Direction::Up {
                        snake.change_direction(Direction::Down);
                    }
                }
                KeyEvent::Pressed(Key::Left) => {
                    if snake.direction() != &Direction::Right {
                        snake.change_direction(Direction::Left);
                    }
                }
                KeyEvent::Pressed(Key::Right) => {
                    if snake.direction() != &Direction::Left {
                        snake.change_direction(Direction::Right);
                    }
                }
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }

        snake.update(&mut board);
        snake.update_movement(&mut board);

        let mut pencil = Pencil::new(window.canvas_mut());

        for cell in board.cells() {
            let c = match cell.cell_type() {
                CellType::Empty => ' ',
                CellType::Food => '*',
                CellType::Snake => '=',
                CellType::SnakeHead => '+',
            };
            let p = Vec2::xy(cell.col(), cell.row());

            match cell.cell_type() {
                CellType::Snake => {
                    pencil.set_foreground(Color::Green);
                }
                CellType::SnakeHead => {
                    pencil.set_foreground(Color::Green);
                }
                CellType::Food => {
                    pencil.set_foreground(Color::Blue);
                }
                _ => {
                    pencil.set_foreground(Color::White);
                }
            };

            pencil.draw_char(c, p);
        }
    });
}
