#[cfg(test)]
mod my_test_map_builder {
    use crate::model::map_builder::MapBuilder;
    use crate::model::random_number_generator::PreProgrammedRandomNumbers;
    use crate::shared_types::TileType;
    use bracket_lib::geometry::Point;
    use std::collections::VecDeque;

    #[test]
    fn test_basic() {
        let mut rng = PreProgrammedRandomNumbers::new(VecDeque::from([0.1f32, 0.2f32, 0.3f32, 0.4f32, 0.5f32, 0.6f32, 0.7f32, 0.8f32]));
        let mp = MapBuilder::new(10, 10, 2, 1, 2, &mut rng);
        assert_eq!(mp.rooms.len(), 2);
    }

    #[test]
    fn test_2_rooms() {
        let mut rng = PreProgrammedRandomNumbers::new(VecDeque::from([
            0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.5f32, 0.5f32, 1.0f32, 1.0f32,
        ]));
        const MIN_ROOM_SIZE: usize = 2;
        const MAX_ROOM_SIZE: usize = 10;
        let mut mp = MapBuilder::new(100, 100, 2, MIN_ROOM_SIZE, MAX_ROOM_SIZE, &mut rng);
        mp.build_random_rooms(2);
        assert_eq!(mp.rooms.len(), 2);

        let room0 = mp.rooms[0];
        assert_eq!(room0.x1, 1);
        assert_eq!(room0.y1, 1);
        assert_eq!(room0.width(), MIN_ROOM_SIZE.try_into().unwrap());
        assert_eq!(room0.height(), MIN_ROOM_SIZE.try_into().unwrap());

        let room1 = mp.rooms[1];
        assert_eq!(room1.x1, 45);
        assert_eq!(room1.y1, 45);
        assert_eq!(room1.width(), MAX_ROOM_SIZE.try_into().unwrap());
        assert_eq!(room1.height(), MAX_ROOM_SIZE.try_into().unwrap());
    }
    //
    // #[test]
    // fn test_vertical_tunnel() {
    //     let mut rng =
    //         PreProgrammedRandomNumbers::new(VecDeque::from([0.5f32, 0.5f32, 1.0f32, 1.0f32]));
    //     let mut mp = MapBuilder::new(100, 100, 2, 2, 10, &mut rng);
    //     mp.apply_vertical_tunnel(44, 46, 3);
    //     for x in 0..100 {
    //         for y in 0..100 {
    //             let point = Point { x, y };
    //             let is_tunnel = x == 3 && y >= 44 && y <= 46;
    //             let tile = mp.map.get_tile(point);
    //             if is_tunnel {
    //                 assert_eq!(tile, TileType::Floor);
    //             } else {
    //                 assert_eq!(tile, TileType::Wall);
    //             }
    //         }
    //     }
    // }
    //
    // #[test]
    // fn test_horizontal_tunnel() {
    //     let mut rng =
    //         PreProgrammedRandomNumbers::new(VecDeque::from([0.5f32, 0.5f32, 1.0f32, 1.0f32, 0.5f32, 0.5f32, 1.0f32, 1.0f32]));
    //     let mut mp = MapBuilder::new(100, 100, 2, 2, 10, &mut rng);
    //     mp.apply_vertical_tunnel(44, 46, 3);
    //     for x in 0..100 {
    //         for y in 0..100 {
    //             let point = Point { x, y };
    //             let is_tunnel = x == 3 && y >= 44 && y <= 46;
    //             let tile = mp.map.get_tile(point);
    //             if is_tunnel {
    //                 assert_eq!(tile, TileType::Floor);
    //             } else {
    //                 assert_eq!(tile, TileType::Wall);
    //             }
    //         }
    //     }
    // }

    #[test]
    fn test_build_corridors() {
        let mut rng = PreProgrammedRandomNumbers::new(VecDeque::from([
            0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.5f32, 0.5f32, 1.0f32, 1.0f32,
        ]));
        const MIN_ROOM_SIZE: usize = 2;
        const MAX_ROOM_SIZE: usize = 10;
        let mut mp = MapBuilder::new(100, 100, 2, MIN_ROOM_SIZE, MAX_ROOM_SIZE, &mut rng);
        mp.build_random_rooms(2);
        assert_eq!(mp.rooms.len(), 2);

        let room0 = mp.rooms[0];
        assert_eq!(room0.x1, 1);
        assert_eq!(room0.y1, 1);
        assert_eq!(room0.width(), MIN_ROOM_SIZE.try_into().unwrap());
        assert_eq!(room0.height(), MIN_ROOM_SIZE.try_into().unwrap());

        let room1 = mp.rooms[1];
        assert_eq!(room1.x1, 45);
        assert_eq!(room1.y1, 45);
        assert_eq!(room1.width(), MAX_ROOM_SIZE.try_into().unwrap());
        assert_eq!(room1.height(), MAX_ROOM_SIZE.try_into().unwrap());

        /*
        x -------------->
        y Y/X| 0| 1| 2| 3| 4| 5| 6|..|43|44|45|..|54|55|
        | 0     |  |  |  |  |  |  |  |  |  |  |  |  |  |
        | 1     | F| F|  |  |  |  |  |  |  |  |  |  |  |
        | 2     | F| F|  |  |  |  |  |  |  |  |  |  |  |
        | 3     |  |  |  |  |  |  |  |  |  |  |  |  |  |
        | 4     |  |  |  |  |  |  |  |  |  |  |  |  |  |
        | ..    |  |  |  |  |  |  |  |  |  |  |  |  |  |
        | 43    |  |  |  |  |  |  |  |  |  |  |  |  |  |
        | 44    |  |  |  |  |  |  |  |  |  |  |  |  |  |
        | 45    |  |  |  |  |  |  |  |  |  | F| F| F|  |
        | ..    |  |  |  |  |  |  |  |  |  | F| F| F|  |
        | 54    |  |  |  |  |  |  |  |  |  | F| F| F|  |
        | 55    |  |  |  |  |  |  |  |  |  |  |  |  |  |
         */
        assert_eq!(mp.map.get_tile(Point { x: 1, y: 1 }), TileType::Floor);
        assert_eq!(mp.map.get_tile(Point { x: 2, y: 2 }), TileType::Floor);
        assert_eq!(mp.map.get_tile(Point { x: 3, y: 3 }), TileType::Wall);
        assert_eq!(mp.map.get_tile(Point { x: 44, y: 44 }), TileType::Wall);
        assert_eq!(mp.map.get_tile(Point { x: 45, y: 45 }), TileType::Floor);
        assert_eq!(mp.map.get_tile(Point { x: 46, y: 46 }), TileType::Floor);
        assert_eq!(mp.map.get_tile(Point { x: 53, y: 53 }), TileType::Floor);
        assert_eq!(mp.map.get_tile(Point { x: 54, y: 54 }), TileType::Floor);
        assert_eq!(mp.map.get_tile(Point { x: 55, y: 55 }), TileType::Wall);

        // these will be corridors
        assert_eq!(mp.map.get_tile(Point { x: 3, y: 1 }), TileType::Wall);
        mp.build_corridors();
        /*
        x -------------->
        y Y/X| 0| 1| 2| 3| 4| 5| 6|..|43|44|45|..|54|55|
        | 0     |  |  |  |  |  |  |  |  |  |  |  |  |  |
        | 1     | F| F|  |  |  |  |  |  |  |  |  |  |  |
        | 2     | F| F|  |  |  |  |  |  |  |  |  |  |  |
        | 3     |  |  |  |  |  |  |  |  |  |  |  |  |  |
        | 4     |  |  |  |  |  |  |  |  |  |  |  |  |  |
        | ..    |  |  |  |  |  |  |  |  |  |  |  |  |  |
        | 43    |  |  |  |  |  |  |  |  |  |  |  |  |  |
        | 44    |  |  |  |  |  |  |  |  |  |  |  |  |  |
        | 45    |  |  |  |  |  |  |  |  |  | F| F| F|  |
        | ..    |  |  |  |  |  |  |  |  |  | F| F| F|  |
        | 54    |  |  |  |  |  |  |  |  |  | F| F| F|  |
        | 55    |  |  |  |  |  |  |  |  |  |  |  |  |  |
         */
        assert_eq!(mp.map.get_tile(Point { x: 3, y: 2 }), TileType::Floor);
        assert_eq!(mp.map.get_tile(Point { x: 4, y: 2 }), TileType::Floor);
        assert_eq!(mp.map.get_tile(Point { x: 45, y: 2 }), TileType::Floor);
        assert_eq!(mp.map.get_tile(Point { x: 50, y: 2 }), TileType::Floor);
        assert_eq!(mp.map.get_tile(Point { x: 51, y: 2 }), TileType::Wall);
        assert_eq!(mp.map.get_tile(Point { x: 5, y: 1 }), TileType::Wall);
        assert_eq!(mp.map.get_tile(Point { x: 5, y: 3 }), TileType::Wall);

        assert_eq!(mp.map.get_tile(Point { x: 50, y: 3 }), TileType::Floor);
        assert_eq!(mp.map.get_tile(Point { x: 50, y: 4 }), TileType::Floor);
        assert_eq!(mp.map.get_tile(Point { x: 50, y: 44 }), TileType::Floor);
        assert_eq!(mp.map.get_tile(Point { x: 50, y: 45 }), TileType::Floor);
        assert_eq!(mp.map.get_tile(Point { x: 50, y: 55 }), TileType::Wall);
        assert_eq!(mp.map.get_tile(Point { x: 49, y: 44 }), TileType::Wall);
        assert_eq!(mp.map.get_tile(Point { x: 51, y: 44 }), TileType::Wall);
    }
}
