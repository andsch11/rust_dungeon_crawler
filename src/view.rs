use crate::model::{GetPlayerPosition, GetTile, InBounds};
use bracket_lib::geometry::Point;
use bracket_lib::prelude::*;

use crate::shared_types::TileType;
use crate::shared_types::{CameraView, Movement};

pub struct View {
    pub width: usize,
    pub height: usize,
}

impl View {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
    pub fn cls(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0); // 0 : first layer : base map
        ctx.cls();
        ctx.set_active_console(1); // 1 : second layer : player
        ctx.cls();
    }
    pub fn get_keycode_movement(&self, ctx: &mut BTerm) -> Movement {
        let key = ctx.key;
        if key.is_none() {
            return Movement::None;
        }
        match key.unwrap() {
            VirtualKeyCode::Left => Movement::Left,
            VirtualKeyCode::Right => Movement::Right,
            VirtualKeyCode::Up => Movement::Up,
            VirtualKeyCode::Down => Movement::Down,
            _ => Movement::None,
        }
    }
    pub fn render_map(
        &mut self,
        ctx: &mut BTerm,
        model: &impl GetTile,
        camera: &CameraView,
        in_bounds_provider: &impl InBounds,
    ) {
        ctx.set_active_console(0); // 0 : first layer : base map
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                let point = Point::new(x, y);
                if !in_bounds_provider.is_in_map_bounds(point) {
                    break;
                }
                let tile = model.get_tile(Point {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                });
                match tile {
                    TileType::Floor => {
                        ctx.set(
                            x - camera.left_x,
                            y - camera.top_y,
                            YELLOW,
                            BLACK,
                            to_cp437('.'),
                        );
                    }
                    TileType::Wall => {
                        ctx.set(
                            x - camera.left_x,
                            y - camera.top_y,
                            GREEN,
                            BLACK,
                            to_cp437('#'),
                        );
                    }
                }
            }
        }
    }

    pub fn render_player(
        &mut self,
        ctx: &mut BTerm,
        model: &impl GetPlayerPosition,
        camera: &CameraView,
    ) {
        ctx.set_active_console(1); // 1 : second layer : player
        let pos = model.get_player_position();
        ctx.set(
            pos.x - camera.left_x,
            pos.y - camera.top_y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }
}
