use crate::model::map::Map;
use crate::model::random_number_generator::GenerateRandomNumber;

use crate::shared_types::TileType;

use bracket_lib::geometry::{Point, Rect};

pub struct MapBuilder<'a> {
    pub map: Map,
    pub player_start: Point,
    pub rooms: Vec<Rect>,
    rng: &'a mut dyn GenerateRandomNumber,
    width: usize,
    height: usize,
    min_room_size: usize,
    max_room_size: usize,
}

impl<'a> MapBuilder<'a> {
    pub fn new(
        width: usize,
        height: usize,
        room_count: usize,
        min_room_size: usize,
        max_room_size: usize,
        rng: &'a mut dyn GenerateRandomNumber,
    ) -> Self {
        assert!(min_room_size < width);
        assert!(min_room_size < height);
        assert!(max_room_size < width);
        assert!(max_room_size < height);
        assert!(min_room_size < max_room_size);

        let mut mb = MapBuilder {
            map: Map::new(
                width,
                height,
                vec![TileType::Wall; (width * height) as usize],
            ),
            player_start: Point::zero(),
            rooms: Vec::new(),
            rng,
            width,
            height,
            min_room_size,
            max_room_size,
        };
        mb.build_random_rooms(room_count);
        mb.build_corridors();
        mb.player_start = mb.rooms[0].center();
        mb
    }

    pub fn build_random_rooms(&mut self, room_count: usize) {
        assert!(self.width > self.max_room_size);
        assert!(self.height > self.min_room_size);
        'create_rooms: while self.rooms.len() < room_count {
            let new_room = Rect::with_size(
                self.rng.range(1, self.width - self.max_room_size),
                self.rng.range(1, self.height - self.max_room_size),
                self.rng.range(self.min_room_size, self.max_room_size),
                self.rng.range(self.min_room_size, self.max_room_size),
            );
            for already_existing_room in self.rooms.iter() {
                if new_room.intersect(already_existing_room) {
                    continue 'create_rooms;
                }
            }
            new_room.for_each(|p| {
                assert!(p.x >= 0);
                assert!(p.y >= 0);
                assert!((p.x as usize) < self.width);
                assert!((p.y as usize) < self.height);
                let idx = self.map.map_idx(p);
                self.map.tiles[idx] = TileType::Floor;
            });
            self.rooms.push(new_room);
        }
    }

    // fixme pub for test
    pub fn apply_vertical_tunnel(&mut self, y1: usize, y2: usize, x: usize) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.map.try_idx(Point {
                x: x.try_into().unwrap(),
                y: y.try_into().unwrap(),
            });
            if idx.is_none() {
                continue;
            }
            self.map.tiles[idx.unwrap()] = TileType::Floor;
        }
    }

    pub fn apply_horizontal_tunnel(&mut self, x1: usize, x2: usize, y: usize) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.map.try_idx(Point {
                x: x.try_into().unwrap(),
                y: y.try_into().unwrap(),
            });
            if idx.is_none() {
                continue;
            }
            self.map.tiles[idx.unwrap()] = TileType::Floor;
        }
    }
    pub fn build_corridors(&mut self) {
        assert!(self.rooms.len() > 1);
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();
            self.apply_horizontal_tunnel(
                prev.x.try_into().unwrap(),
                new.x.try_into().unwrap(),
                prev.y.try_into().unwrap(),
            );
            self.apply_vertical_tunnel(
                prev.y.try_into().unwrap(),
                new.y.try_into().unwrap(),
                new.x.try_into().unwrap(),
            );
        }
    }
}
