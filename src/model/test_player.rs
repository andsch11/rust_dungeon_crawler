#[cfg(test)]
mod my_test_player_module_name_fixme {
    use crate::model::player::Player;
    use bracket_lib::geometry::Point;

    #[test]
    fn test_basic() {
        let player = Player::new(Point { x: 3, y: 4 });
        assert_eq!(player.get_position(), Point { x: 3, y: 4 });
    }

    #[test]
    fn test_set_get_position() {
        let mut player = Player::new(Point { x: 3, y: 4 });
        assert_eq!(player.get_position(), Point { x: 3, y: 4 });
        let test_position = Point { x: 34, y: 89 };
        player.set_position(test_position);
        assert_eq!(player.get_position(), test_position);
    }
}
