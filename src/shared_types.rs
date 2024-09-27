#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TileType {
    Wall,
    Floor,
}

pub enum Movement {
    Left,
    Right,
    Up,
    Down,
    None,
}

pub struct CameraView {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}
