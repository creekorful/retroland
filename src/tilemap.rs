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

        TileMap {
            tiles,
            size
        }
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
}
