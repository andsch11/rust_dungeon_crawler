// items used within 'model' and are used later for public interfaces.
// FIXME is this needed to make cargo aware that these modules are there and need to be compiled ???
mod game_logic;
mod map;
mod player;
mod map_builder;
mod random_number_generator;

// public interface of this module
pub use game_logic::GetPlayerPosition;
pub use game_logic::GetTile;
pub use game_logic::Model;

// test helpers
mod test_game_logic;
mod test_map;
mod test_map_builder;
mod test_player;
mod test_random_number_generator;
