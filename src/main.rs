mod model;
mod presenter;
mod shared_types;
mod view;

const MODEL_WIDTH: usize = 80;
const MODEL_HEIGHT: usize = 40;
const DISPLAY_WIDTH: usize = MODEL_WIDTH / 2;
const DISPLAY_HEIGHT: usize = MODEL_HEIGHT / 2;
const ROOM_COUNT: usize = 10;
const ROOM_MIN_SIZE: usize = 2;
const ROOM_MAX_SIZE: usize = 10;

use bracket_lib::prelude::*;

use crate::model::*;
use crate::presenter::*;
use crate::view::*;

fn main() {
    println!("Hello, world!");
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()
        .unwrap();
    // these lines we need later when camera view is working
    // let model = Model::new(SCREEN_WIDTH, SCREEN_HEIGHT, None, None);
    // let view = View::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let model = Model::new()
        .default(
            MODEL_WIDTH,
            MODEL_HEIGHT,
            ROOM_COUNT,
            ROOM_MIN_SIZE,
            ROOM_MAX_SIZE,
        )
        .build();
    let view = View::new(DISPLAY_WIDTH, DISPLAY_HEIGHT);
    let _ = main_loop(context, Presenter::new(model, view));
}
