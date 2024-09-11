#[cfg(test)]
mod my_test_model_module_name_fixme {
    use crate::model::GetPlayerPosition;
    use crate::model::Model;
    use crate::shared_types::Movement;
    use crate::shared_types::TileType;
    use bracket_lib::geometry::Point;
    use crate::model::map::Map;

    #[test]
    fn test_basic() {
        let _ = Model::new().default(22, 33, 2, 1, 5).build();
    }

    #[test]
    fn test_none_movement() {
        let mut model = Model::new().default(22, 33, 2, 1, 10).build();
        let start_position = model.get_player_position();
        model.game_logic_update(Movement::None);
        assert_eq!(start_position, model.get_player_position());
    }

    #[test]
    fn test_4basic_movements() {
        // works only for tiles and if in boundaries
        let tiles = vec![
            TileType::Floor,
            TileType::Floor,
            TileType::Floor,
            TileType::Floor,
        ];
 
        let map = Map::new(2, 2, tiles);
        let mut model = Model::new().custom_map(map, Point::new(0, 0)).build();
        let start_position = model.get_player_position();
        model.game_logic_update(Movement::Down);
        assert_eq!(
            start_position + Point::new(0, 1),
            model.get_player_position()
        );
        model.game_logic_update(Movement::Right);
        assert_eq!(
            start_position + Point::new(1, 1),
            model.get_player_position()
        );
        model.game_logic_update(Movement::Up);
        assert_eq!(
            start_position + Point::new(1, 0),
            model.get_player_position()
        );
        model.game_logic_update(Movement::Left);
        assert_eq!(start_position, model.get_player_position());
    }

    #[test]
    fn test_not_allowed_movements_wall_tiles() {
        let tiles = vec![
            //                  x,y
            TileType::Floor, // 0,0
            TileType::Wall,  // 1,0
            TileType::Floor,
            TileType::Floor,
            TileType::Floor,
            TileType::Floor, // 4,0
            TileType::Floor, // 0,1
            TileType::Wall,
            TileType::Floor,
            TileType::Floor, // 4,1
            TileType::Floor, // 0,2
            TileType::Floor,
            TileType::Floor,
            TileType::Wall,
            TileType::Floor, // 4,2
        ];
        // Example of 5x3 Map
        //               W...all
        //           x  0  1  2  3  4
        // H  y axis  +--+--+--+--+--+
        // E    |     | 0| W| 2| 3| 4|  y=0
        // I    |     +--+--+--+--+--+
        // G    V     | 5| 6| W| 8| 9|  y=1
        // H          +--+--+--+--+--+
        // T          |10|11|12| W|14|  y=2
        //            +--+--+--+--+--+
        // x axis------------------------>
        //            <<<< WIDTH >>>>

        assert_eq!(tiles.len(), 5 * 3);
        let map = Map::new(5, 3, tiles);
        let mut model = Model::new().custom_map(map, Point::new(1, 1)).build();
        let start_position = model.get_player_position();
        assert_eq!(start_position, Point::new(1, 1));
        model.game_logic_update(Movement::Down);
        assert_eq!(model.get_player_position(), Point { x: 1, y: 2 });
        model.game_logic_update(Movement::Right);
        assert_eq!(model.get_player_position(), Point { x: 2, y: 2 });
        // Position will not change now . Walls Above, to the right, and end of world below
        model.game_logic_update(Movement::Up);
        assert_eq!(model.get_player_position(), Point { x: 2, y: 2 });
        model.game_logic_update(Movement::Right);
        assert_eq!(model.get_player_position(), Point { x: 2, y: 2 });
        model.game_logic_update(Movement::Down);
        assert_eq!(model.get_player_position(), Point { x: 2, y: 2 });

        model.game_logic_update(Movement::Left);
        assert_eq!(model.get_player_position(), Point { x: 1, y: 2 });
    }

    #[test]
    fn test_not_allowed_movements_outside_boundaries() {
        let tiles = vec![
            //                  x,y
            TileType::Floor, // 0,0
        ];

        assert_eq!(tiles.len(), 1 * 1);
        let map = Map::new(1, 1, tiles);
        let mut model = Model::new().custom_map(map, Point::new(0, 0)).build();
        let start_position = model.get_player_position();
        assert_eq!(start_position, Point::new(0, 0));
        model.game_logic_update(Movement::Down);
        assert_eq!(model.get_player_position(), start_position);
        model.game_logic_update(Movement::Right);
        assert_eq!(model.get_player_position(), start_position);
        model.game_logic_update(Movement::Up);
        assert_eq!(model.get_player_position(), start_position);
        model.game_logic_update(Movement::Down);
        assert_eq!(model.get_player_position(), start_position);
    }
}
