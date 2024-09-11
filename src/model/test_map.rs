#[cfg(test)]
mod my_test_map {
    use crate::model::map::Map;
    use crate::shared_types::TileType;
    use bracket_lib::geometry::Point;

    #[test]
    fn test_basic() {
        let _ = Map::new(3, 5, vec![TileType::Floor; (3 * 5) as usize]);
    }

    // TODO Free function,  no map instance needed
    #[test]
    fn map_idx() {
        let map = Map::new(10, 5, vec![TileType::Floor; (10 * 5) as usize]);
        assert_eq!(map.map_idx(Point { x: 0, y: 0 }), 0);
        assert_eq!(map.map_idx(Point { x: 1, y: 0 }), 1);
        assert_eq!(map.map_idx(Point { x: 2, y: 0 }), 2);
        assert_eq!(map.map_idx(Point { x: 0, y: 1 }), 10 as usize);
        assert_eq!(
            map.map_idx(Point { x: 1, y: 1 }),
            (map.get_width() + 1) as usize
        );
    }

    // TODO Free function,  no map instance needed
    #[test]
    fn in_bounds() {
        let map = Map::new(10, 5, vec![TileType::Floor; (10 * 5) as usize]);
        let max_x: i32 = map.get_width() as i32;
        let max_y: i32 = map.get_height() as i32;
        assert!(map.in_bounds(Point { x: 0, y: 0 }));
        assert!(map.in_bounds(Point { x: max_x - 1, y: 0 }));
        assert!(!map.in_bounds(Point { x: max_x, y: 0 }));
        assert!(map.in_bounds(Point { x: 0, y: max_y - 1 }));
        assert!(!map.in_bounds(Point { x: 0, y: max_y }));
        assert!(!map.in_bounds(Point { x: max_x, y: max_y }));
        assert!(map.in_bounds(Point {
            x: max_x - 1,
            y: max_y - 1
        }));
    }

    #[test]
    fn get_tile() {
        let map = Map::new(10, 5, vec![TileType::Floor; (10 * 5) as usize]);
        assert_eq!(map.get_tile(Point { x: 0, y: 0 }), TileType::Floor {});
    }

    #[test]
    fn can_enter_tile() {
        let tiles = vec![
            TileType::Floor,
            TileType::Wall,
            TileType::Floor,
            TileType::Floor,
            TileType::Floor,
            TileType::Floor,
            TileType::Floor,
            TileType::Wall,
            TileType::Floor,
            TileType::Floor,
            TileType::Floor,
            TileType::Floor,
            TileType::Floor,
            TileType::Wall,
            TileType::Floor,
        ];
        let map = Map::new(5, 3, tiles);
        for x in 0..5 {
            for y in 0..3 {
                let p = Point { x, y };
                let wall = (p == Point { x: 1, y: 0 })
                    || (p == Point { x: 2, y: 1 })
                    || (p == Point { x: 3, y: 2 });
                assert_ne!(
                    map.can_enter_tile(p),
                    wall,
                    "Failed for test point {} {}",
                    x,
                    y
                );
            }
        }
    }
}
