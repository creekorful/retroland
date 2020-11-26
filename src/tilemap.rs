use sfml::system::Vector2u;

/// TileMap is the raw representation of a tile map
struct TileMap {
    /// the map tiles, the first vector is the layer, the second is the tiles in row major order
    tiles: Vec<Vec<u32>>,
    /// the tile map size
    size: Vector2u,
}

impl TileMap {
    /// Create a new tile map of given size, with given number of layers
    /// The initial layers will be fill with 1 (grass)
    /// the others will be fill with 0 (air)
    fn new<T: Into<Vector2u>>(size: T, nb_layers: u32) -> Self {
        let size = size.into();
        let mut tiles = Vec::with_capacity(nb_layers as usize);

        tiles.push(vec![1; (size.x * size.y) as usize]);

        for _ in 1..nb_layers {
            tiles.push(vec![0; (size.x * size.y) as usize]);
        }

        TileMap { tiles, size }
    }

    /// Retrieve the tile at given position on given layer
    /// this will return None if the position / layers doesn't exist
    fn get_tile<T: Into<Vector2u>>(&self, position: T, layer: u32) -> Option<u32> {
        let position = position.into();

        // Validate input
        if position.x >= self.size.x || position.y >= self.size.y {
            return None;
        }

        self.tiles
            .get(layer as usize)
            .and_then(|v| v.get((position.x + (position.y * self.size.x)) as usize))
            .copied()
    }
}

#[cfg(test)]
mod tests {
    use crate::tilemap::TileMap;

    #[test]
    fn test_tile_map_new() {
        let tile_map = TileMap::new((20, 10), 2);

        assert_eq!(tile_map.tiles.len(), 2);
        assert_eq!(tile_map.size.x, 20);
        assert_eq!(tile_map.size.y, 10);
        assert_eq!(tile_map.tiles.get(0).unwrap().len(), 20 * 10);
        assert_eq!(tile_map.tiles.get(1).unwrap().len(), 20 * 10);

        // Make sure first layer is fill with 1
        for i in 0..200 {
            assert_eq!(tile_map.tiles[0][i], 1);
        }

        // Make sure second layer is fill with 0
        for i in 0..200 {
            assert_eq!(tile_map.tiles[1][i], 0);
        }
    }

    #[test]
    fn test_tile_map_get_tile() {
        let tile_map = TileMap::new((20, 10), 2);

        for layer in 0..2 {
            for y in 0..10 {
                for x in 0..20 {
                    let expected_value = if layer == 0 { 1 } else { 0 };
                    assert_eq!(tile_map.get_tile((x, y), layer).unwrap(), expected_value);
                }
            }
        }

        // Make sure access to non-existing value returns None
        assert!(tile_map.get_tile((30, 5), 0).is_none()); // position not valid
        assert!(tile_map.get_tile((0, 0), 22).is_none()); // layer doesn't exist
    }
}
