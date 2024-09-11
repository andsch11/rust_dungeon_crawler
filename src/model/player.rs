use bracket_lib::geometry::Point;

pub struct Player {
    position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }
    pub fn get_position(&self) -> Point {
        self.position
    }

    pub fn set_position(&mut self, position: Point) {
        self.position = position;
    }
}
