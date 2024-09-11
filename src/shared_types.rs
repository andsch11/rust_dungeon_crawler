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
