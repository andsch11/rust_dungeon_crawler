use crate::shared_types::TileType;

use bracket_lib::geometry::Point;
// TODO derive from Debug needed for asserts in UnitTests. What is the overhead coming on that for e.g. embedded.

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub num_tiles: usize,
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new(width: usize, height: usize, tiles: Vec<TileType>) -> Self {
        assert_eq!(tiles.len(), (width * height));
        Self {
            width: width,
            height: height,
            num_tiles: height * width,
            tiles: tiles,
        }
    }

    pub fn get_tile(&self, point: Point) -> TileType {
        let idx = self.map_idx(point);
        self.tiles[idx]
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        if point.x < 0 {
            return false;
        }
        if point.x >= self.get_width() as i32 {
            return false;
        }
        if point.y < 0 {
            return false;
        }
        if point.y >= self.get_height() as i32 {
            return false;
        }
        true
    }
    pub fn can_enter_tile(&self, point: Point) -> bool {
        if !self.in_bounds(point) {
            return false;
        }
        let idx = self.map_idx(point);
        self.tiles[idx] == TileType::Floor
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(self.map_idx(point))
        }
    }

    pub fn map_idx(&self, point: Point) -> usize {
        assert!(point.x >= 0);
        assert!(point.y >= 0);
        // we can safely cast point to usize now
        assert!((point.x as usize) < self.width);
        assert!((point.y as usize) < self.height);
        let tile_index = ((point.x as usize) + ((point.y as usize) * self.width)) as usize;
        assert!(tile_index < self.num_tiles as usize);
        tile_index
    }
}

// Example of 5x3 Map
// H  y axis  +--+--+--+--+--+
// E    |     | 0| 1| 2| 3| 4|
// I    |     +--+--+--+--+--+
// G    V     | 5| 6| 7| 8| 9|
// H          +--+--+--+--+--+
// T          |10|11|12|13|14|
//            +--+--+--+--+--+
// x axis------------------------>
//            <<<< WIDTH >>>>
