use crate::model::map::Map;
use crate::model::map_builder::MapBuilder;
use crate::model::player::Player;
use crate::model::random_number_generator::RealRng;

use crate::shared_types::Movement;
use crate::shared_types::TileType;

use bracket_lib::geometry::Point;

// fixme can i name this file model.rs as well (same as in outer foler) ?

pub trait GetTile {
    fn get_tile(&self, point: Point) -> TileType;
}

pub trait GetPlayerPosition {
    fn get_player_position(&self) -> Point;
}

pub trait InBounds {
    fn is_in_map_bounds(&self, point: Point) -> bool;
}

pub struct Model {
    map: Map,
    player: Player,
}

// internal stuff

impl GetTile for Model {
    fn get_tile(&self, point: Point) -> TileType {
        self.map.get_tile(point)
    }
}
impl GetPlayerPosition for Model {
    fn get_player_position(&self) -> Point {
        self.player.get_position()
    }
}

impl InBounds for Model {
    fn is_in_map_bounds(&self, point: Point) -> bool {
        self.map.in_bounds(point)
    }
}

impl Model {
    pub fn new() -> ModelBuilder {
        ModelBuilder::new()
    }
    pub fn game_logic_update(&mut self, movement: Movement) {
        let delta = match movement {
            Movement::Left => Point::new(-1, 0),
            Movement::Right => Point::new(1, 0),
            Movement::Down => Point::new(0, 1),
            Movement::Up => Point::new(0, -1),
            Movement::None => Point::zero(),
        };
        let new_position = self.player.get_position() + delta;
        if self.map.can_enter_tile(new_position) {
            self.player.set_position(new_position);
        }
    }
}

pub struct ModelBuilder {
    map: Option<Map>,
    player_starting_point: Option<Point>,
}

// https://rust-unofficial.github.io/patterns/patterns/creational/builder.html
impl ModelBuilder {
    pub fn new() -> ModelBuilder {
        ModelBuilder {
            map: None,
            player_starting_point: None,
        }
    }

    pub fn default(
        mut self,
        width: usize,
        height: usize,
        room_count: usize,
        min_room_size: usize,
        max_room_size: usize,
    ) -> ModelBuilder {
        // let mut rng = PreProgrammedRandomNumbers::new(VecDeque::from([
        //     0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.5f32, 0.5f32, 1.0f32, 1.0f32,
        // ]));
        let mut rng = RealRng::new();
        let map_builder = MapBuilder::new(
            width,
            height,
            room_count,
            min_room_size,
            max_room_size,
            &mut rng,
        );
        self.map = Option::from(map_builder.map);
        self.player_starting_point = Option::from(map_builder.player_start);
        self
    }

    #[allow(dead_code)]
    pub fn custom_map(mut self, map: Map, player_starting_point: Point) -> ModelBuilder {
        self.map = Option::from(map);
        self.player_starting_point = Option::from(player_starting_point);
        self
    }

    pub fn build(self) -> Model {
        assert!(self.map.is_some());
        assert!(self.player_starting_point.is_some());

        Model {
            map: self.map.unwrap(),
            player: Player::new(self.player_starting_point.unwrap()),
        }
    }
}
