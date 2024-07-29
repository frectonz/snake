use kira::{
    manager::{backend::cpal::CpalBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::StaticSoundData,
};
use ruscii::{
    app::{App, Config, State},
    drawing::Pencil,
    gui::FPSCounter,
    keyboard::{Key, KeyEvent},
    spatial::Vec2,
    terminal::{Color, Window},
};
use snake::{Board, CellType, Direction, GameEvent, Snake};
use std::{io::Cursor, sync::mpsc, time::Instant};

const EAT_SOUND: &[u8] = include_bytes!("../../assets/eat.mp3");
const DIE_SOUND: &[u8] = include_bytes!("../../assets/die.mp3");
const MOVE_SOUND: &[u8] = include_bytes!("../../assets/move.mp3");

struct Game {
    board: Board,
    snake: Snake,
    fps_counter: FPSCounter,
    offset: usize,
    center: usize,
    start: Instant,
}

impl Game {
    fn game_over_keybindings(&mut self, app_state: &mut State) {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Space) => {
                    self.board.reset();
                    self.snake.reset();
                }
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            };
        }
    }

    fn paused_keybindings(&mut self, app_state: &mut State) {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => {
                    self.board.toggle_pause();
                }
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            };
        }
    }

    fn game_keybindings(&mut self, app_state: &mut State) {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Up) | KeyEvent::Pressed(Key::W) => {
                    self.snake.change_direction(Direction::Up, &mut self.board);
                }
                KeyEvent::Pressed(Key::Down) | KeyEvent::Pressed(Key::S) => {
                    self.snake
                        .change_direction(Direction::Down, &mut self.board);
                }
                KeyEvent::Pressed(Key::Left) | KeyEvent::Pressed(Key::A) => {
                    self.snake
                        .change_direction(Direction::Left, &mut self.board);
                }
                KeyEvent::Pressed(Key::Right) | KeyEvent::Pressed(Key::D) => {
                    self.snake
                        .change_direction(Direction::Right, &mut self.board);
                }
                KeyEvent::Pressed(Key::Esc) => {
                    self.board.toggle_pause();
                }
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }
    }

    fn draw_stats(&mut self, pencil: &mut Pencil) {
        pencil.draw_text(
            &format!("Score: {}", self.snake.score()),
            Vec2::xy(self.offset, self.board.rows() + self.offset + 1),
        );
        pencil.draw_text(
            &format!("FPS: {}", self.fps_counter.count()),
            Vec2::xy(self.offset, self.board.rows() + self.offset + 2),
        );
        pencil.draw_text(
            &format!("{} seconds", self.start.elapsed().as_secs()),
            Vec2::xy(self.offset, self.board.rows() + self.offset + 3),
        );
    }

    fn draw_game_over_header(&mut self, pencil: &mut Pencil) {
        pencil.set_foreground(Color::Red);
        pencil.draw_center_text("GAME OVER", Vec2::xy(self.center, 1));
        pencil.set_foreground(Color::White);
        pencil.draw_center_text(
            "Press <space> to restart the game",
            Vec2::xy(self.center, 2),
        );
        pencil.draw_center_text("Press <q> to quit the game", Vec2::xy(self.center, 3));
    }

    fn draw_game_header(&mut self, pencil: &mut Pencil) {
        pencil.set_foreground(Color::Green);
        pencil.draw_center_text("SNAKE", Vec2::xy(self.center, 1));
        pencil.set_foreground(Color::White);
        pencil.draw_center_text("Press <q> to quit the game", Vec2::xy(self.center, 2));
        pencil.draw_center_text("Press <ESC> to pause the game", Vec2::xy(self.center, 3));
        pencil.draw_center_text(
            "Use arrow keys or <wasd> for movement",
            Vec2::xy(self.center, 4),
        );
    }

    fn draw_paused_header(&mut self, pencil: &mut Pencil) {
        pencil.set_foreground(Color::Blue);
        pencil.draw_center_text("PAUSED", Vec2::xy(self.center, 1));
        pencil.set_foreground(Color::White);
        pencil.draw_center_text("Press <q> to quit the game", Vec2::xy(self.center, 2));
        pencil.draw_center_text("Press <ESC> to play the game", Vec2::xy(self.center, 3));
    }

    fn draw_board(&mut self, pencil: &mut Pencil) {
        for cell in self.board.cells() {
            let c = match cell.cell_type() {
                CellType::Empty => '·',
                CellType::Food => ' ',
                CellType::Snake => ' ',
                CellType::SnakeHead => ' ',
            };
            let p = Vec2::xy(cell.col() + self.offset, cell.row() + self.offset);

            match cell.cell_type() {
                CellType::Snake => {
                    pencil.set_background(Color::Green);
                }
                CellType::SnakeHead => {
                    pencil.set_background(Color::Blue);
                }
                CellType::Food => {
                    pencil.set_background(Color::Red);
                }
                CellType::Empty => {
                    pencil.set_background(Color::Black);
                }
            };

            pencil.draw_char(c, p);
        }
    }
}

fn make_sound_data(sound: &'static [u8]) -> StaticSoundData {
    let sound_data_cursor = Cursor::new(sound);
    StaticSoundData::from_cursor(sound_data_cursor).unwrap()
}

fn main() {
    let (tx, rx) = mpsc::sync_channel(1);
    let mut manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default())
        .expect("Failed to create audio manager");

    let eat_sound = make_sound_data(EAT_SOUND);
    let die_sound = make_sound_data(DIE_SOUND);
    let move_sound = make_sound_data(MOVE_SOUND);

    std::thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            match msg {
                GameEvent::FoodEaten => {
                    let _ = manager.play(eat_sound.clone());
                }
                GameEvent::SnakeDied => {
                    let _ = manager.play(die_sound.clone());
                }
                GameEvent::SnakeChangedDirection => {
                    let _ = manager.play(move_sound.clone());
                }
            }
        }
    });

    let config = Config::new().fps(15);
    let mut app = App::config(config);

    let offset = 5;
    let size = app.window().size();
    let cols = size.x.try_into().unwrap_or(15);
    let rows = size.y.try_into().unwrap_or(10);

    let snake = Snake::new();
    let mut board = Board::new(rows - (offset * 2), cols - (offset * 2), Some(tx));

    board.generate_food();

    let fps_counter = FPSCounter::default();
    let center = (board.columns() / 2) + offset;

    let mut game = Game {
        board,
        snake,
        fps_counter,
        offset,
        center,
        start: Instant::now(),
    };

    app.run(|app_state: &mut State, window: &mut Window| {
        game.fps_counter.update();
        let mut pencil = Pencil::new(window.canvas_mut());

        if game.board.game_over() {
            game.draw_game_over_header(&mut pencil);
            game.game_over_keybindings(app_state);
        } else if game.board.paused() {
            game.draw_paused_header(&mut pencil);
            game.paused_keybindings(app_state);
        } else {
            game.draw_game_header(&mut pencil);
            game.game_keybindings(app_state);

            game.snake.update(&mut game.board);
            game.snake.update_movement(&mut game.board);
        }

        game.draw_stats(&mut pencil);
        game.draw_board(&mut pencil);
    });
}
