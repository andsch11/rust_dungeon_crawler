use crate::model::{GetPlayerPosition, GetTile};
use bracket_lib::geometry::Point;
use bracket_lib::prelude::*;

use crate::shared_types::Movement;
use crate::shared_types::TileType;

pub struct View {
    width: usize,
    height: usize,
}

impl View {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
    pub fn cls(&mut self, ctx: &mut BTerm) {
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
    pub fn render_map(&mut self, ctx: &mut BTerm, model: &impl GetTile) {
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = model.get_tile(Point {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                });
                match tile {
                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                    }
                    TileType::Wall => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                    }
                }
            }
        }
    }

    pub fn render_player(&mut self, ctx: &mut BTerm, model: &impl GetPlayerPosition) {
        let pos = model.get_player_position();
        ctx.set(pos.x, pos.y, WHITE, BLACK, to_cp437('@'));
    }
}
