pub mod config {
    use macroquad::window::Conf;

    pub const WINDOW_SIZE: i32 = 1000;
    pub const FPS: u64 = 60;

    pub const SECTOR_WIDTH: f32 = WINDOW_SIZE as f32 / 12.0;

    pub const CLOSE_CALL_DISTANCE: f32 = SECTOR_WIDTH * 0.9;
    pub const COLLISION_DISTANCE: f32 = SECTOR_WIDTH * 0.8;
    pub const SCAN_DISTANCE: f32 = SECTOR_WIDTH * 3.0;
    pub const ACCELERATION_DISTANCE: f32 = SCAN_DISTANCE / 2.0;

    pub const SPEED_LIMIT: f32 = 2.0;
    pub const MAX_VELOCITY: f32 = (SECTOR_WIDTH * SPEED_LIMIT) / FPS as f32;

    pub const CRUISE_SPEED: f32 = SPEED_LIMIT * 0.35;
    pub const MARGIN: f32 = 4.0;
    pub const RANDOM_INTERVAL: u64 = WINDOW_SIZE as u64 / SPEED_LIMIT as u64;
    pub fn window_conf() -> Conf {
        Conf {
            window_title: "Smart-Road | Z01-Dakar".to_owned(),
            window_width: WINDOW_SIZE,
            window_height: WINDOW_SIZE,
            window_resizable: false,
            ..Default::default()
        }
    }
}

pub mod controls {
    use macroquad::prelude::*;

    use crate::circulation::{Direction, State};

    pub fn handle_input(state: &mut State) {
        if is_key_pressed(KeyCode::Escape) && !state.show_final_statistics {
            state.show_final_statistics = true;
        } else if is_key_pressed(KeyCode::Escape) && state.show_final_statistics {
            //std::process::Sortir(0);
        }

        if is_key_pressed(KeyCode::Up) {
            state.add_car(Direction::South);
            state.random = false;
        }

        if is_key_pressed(KeyCode::Down) {
            state.add_car(Direction::North);
            state.random = false;
        }

        if is_key_pressed(KeyCode::Right) {
            state.add_car(Direction::West);
            state.random = false;
        }

        if is_key_pressed(KeyCode::Left) {
            state.add_car(Direction::East);
            state.random = false;
        }

        if is_key_pressed(KeyCode::R) {
            state.random = !state.random;
            //state.add_car_random();
        }
    }
}

pub mod circulation {
    pub use car::*;
    pub use path::*;
    pub use state::{Direction, State};
    pub use statistics::*;

    pub mod car;
    pub mod path;
    pub mod road;
    pub mod state;
    pub mod statistics;

    pub mod collision;
}

pub mod representation {
    pub const FONT_SIZE: f32 = 20.0;
    pub const TITLE_SIZE: f32 = FONT_SIZE * 1.5;

    pub use roads::representation_textured_roads;
    pub use textures::Textures;

    pub mod roads;

    pub mod car;
    pub mod textures;

    pub use car::representation_car;

    pub mod statistics;

    pub use statistics::representation_statistics;
}
