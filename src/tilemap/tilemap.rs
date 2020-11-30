use serde::{Deserialize, Serialize};
use sfml::system::Vector2u;
use std::convert::TryFrom;
use std::fs::File;
use std::io::Write;

#[derive(Debug, PartialEq)]
pub enum TileMapError {
    InvalidPosition,
    InvalidLayer,
    WriteError,
    ReadError,
}

// Allow serde serialization / deserialization of Vector2u
#[derive(Serialize, Deserialize)]
#[serde(remote = "Vector2u")]
struct Vector2uDef {
    x: u32,
    y: u32,
}

/// TileMap is the raw representation of a tile map
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TileMap {
    /// the map tiles, the first vector is the layer, the second is the tiles in row major order
    tiles: Vec<Vec<u32>>,
    /// the tile map size
    #[serde(with = "Vector2uDef")]
    size: Vector2u,
    /// The number of layers
    nb_layers: u32,
}

impl TileMap {
    /// Create a new tile map of given size, with given number of layers
    /// The initial layers will be fill with 2 (grass)
    /// the others will be fill with 0 (air)
    pub fn new<T: Into<Vector2u>>(size: T, nb_layers: u32) -> Self {
        let size = size.into();
        let mut tiles = Vec::with_capacity(nb_layers as usize);

        tiles.push(vec![2; (size.x * size.y) as usize]);

        for _ in 1..nb_layers {
            tiles.push(vec![0; (size.x * size.y) as usize]);
        }

        TileMap {
            tiles,
            size,
            nb_layers,
        }
    }

    /// Retrieve the tile at given position on given layer
    /// this will return None if the position / layers doesn't exist
    pub fn get_tile<T: Into<Vector2u>>(&self, position: T, layer: u32) -> Option<u32> {
        let index = self.compute_index(position.into())?;

        self.tiles
            .get(layer as usize)
            .and_then(|v| v.get(index))
            .copied()
    }

    /// Set the tile at given position and layer
    /// this operation will fails if the position / layer doesn't exist
    pub fn set_tile<T: Into<Vector2u>>(
        &mut self,
        position: T,
        layer: u32,
        tile: u32,
    ) -> Result<(), TileMapError> {
        let index = self
            .compute_index(position.into())
            .ok_or(TileMapError::InvalidPosition)?;

        self.tiles
            .get_mut(layer as usize)
            .ok_or(TileMapError::InvalidLayer)
            .map(|v| v[index] = tile)
    }

    /// Retrieve the tile map size
    pub fn size(&self) -> Vector2u {
        self.size
    }

    /// Retrieve the number of layers
    pub fn nb_layers(&self) -> u32 {
        self.nb_layers
    }

    /// Write the tile map to given writer
    pub fn write(&self, mut writer: impl Write) -> Result<(), TileMapError> {
        let bytes: Vec<u8> = bincode::serialize(&self).map_err(|_| TileMapError::WriteError)?;
        writer
            .write_all(&bytes)
            .map_err(|_| TileMapError::WriteError)
    }

    /// Compute the vector index from given position
    fn compute_index<T: Into<Vector2u>>(&self, position: T) -> Option<usize> {
        let position = position.into();

        // Validate input
        if position.x >= self.size.x || position.y >= self.size.y {
            return None;
        }

        Some((position.x + position.y * self.size.x) as usize)
    }
}

impl TryFrom<File> for TileMap {
    type Error = TileMapError;

    fn try_from(value: File) -> Result<Self, Self::Error> {
        bincode::deserialize_from(value).map_err(|_| TileMapError::ReadError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_map_new() {
        let tile_map = TileMap::new((20, 10), 2);

        assert_eq!(tile_map.tiles.len(), 2);
        assert_eq!(tile_map.size.x, 20);
        assert_eq!(tile_map.size.y, 10);
        assert_eq!(tile_map.nb_layers, 2);
        assert_eq!(tile_map.tiles.get(0).unwrap().len(), 20 * 10);
        assert_eq!(tile_map.tiles.get(1).unwrap().len(), 20 * 10);

        // Make sure first layer is fill with 2
        for i in 0..200 {
            assert_eq!(tile_map.tiles[0][i], 2);
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
                    let expected_value = if layer == 0 { 2 } else { 0 };
                    assert_eq!(tile_map.get_tile((x, y), layer).unwrap(), expected_value);
                }
            }
        }

        // Make sure access to non-existing value returns None
        assert!(tile_map.get_tile((30, 5), 0).is_none()); // position not valid
        assert!(tile_map.get_tile((0, 0), 22).is_none()); // layer doesn't exist
    }

    #[test]
    fn test_tile_map_set_tile() {
        let mut tile_map = TileMap::new((6, 5), 2);

        assert!(tile_map.set_tile((0, 0), 0, 12).is_ok());
        assert_eq!(tile_map.tiles[0][0], 12);
        assert!(tile_map.set_tile((0, 0), 1, 12).is_ok());
        assert_eq!(tile_map.tiles[1][0], 12);

        assert!(tile_map.set_tile((1, 1), 0, 12).is_ok());
        assert_eq!(tile_map.tiles[0][7], 12);
        assert!(tile_map.set_tile((1, 1), 1, 12).is_ok());
        assert_eq!(tile_map.tiles[1][7], 12);

        assert!(tile_map.set_tile((5, 4), 0, 12).is_ok());
        assert_eq!(tile_map.tiles[0][29], 12);
        assert!(tile_map.set_tile((5, 4), 1, 12).is_ok());
        assert_eq!(tile_map.tiles[1][29], 12);

        // check impossible access
        assert_eq!(
            tile_map.set_tile((0, 0), 10, 12).err().unwrap(),
            TileMapError::InvalidLayer
        );
        assert_eq!(
            tile_map.set_tile((70, 0), 1, 12).err().unwrap(),
            TileMapError::InvalidPosition
        );
    }

    #[test]
    fn test_tile_map_compute_index() {
        let tile_map = TileMap::new((6, 5), 2);

        assert_eq!(tile_map.compute_index((0, 0)).unwrap(), 0);
        assert_eq!(tile_map.compute_index((1, 1)).unwrap(), 7);
        assert_eq!(tile_map.compute_index((5, 4)).unwrap(), 29);
        assert!(tile_map.compute_index((70, 0)).is_none());
    }

    #[test]
    fn test_tile_map_size() {
        let tile_map = TileMap::new((20, 10), 2);

        let size = tile_map.size();
        assert_eq!(size.x, 20);
        assert_eq!(size.y, 10);
    }

    #[test]
    fn test_tile_nb_layers() {
        let tile_map = TileMap::new((20, 10), 2);

        assert_eq!(tile_map.nb_layers, 2);
        assert_eq!(tile_map.nb_layers(), 2);
    }
}
