use crate::tilemap::TileMap;
use sfml::graphics::{
    Color, Drawable, RectangleShape, RenderStates, RenderTarget, Shape, Transformable,
};

/// Tile map renderer is used to render a tile map on the screen
pub struct TileMapRenderer<'s> {
    tiles: Vec<RectangleShape<'s>>,
}

impl<'s> TileMapRenderer<'s> {
    pub fn new(tile_map: &TileMap) -> Self {
        let size = tile_map.size();
        let mut tiles = Vec::new();

        // At the moment only draw layer 0
        for y in 0..size.y {
            for x in 0..size.x {
                let mut tile = RectangleShape::new();
                tile.set_size((70.0, 70.0));
                tile.set_position((x as f32 * 70.0, y as f32 * 70.0));
                tile.set_fill_color(Color::RED);
                tile.set_outline_color(Color::BLACK);
                tile.set_outline_thickness(1.0);
                tiles.push(tile);
            }
        }

        TileMapRenderer { tiles }
    }
}

impl<'s> Drawable for TileMapRenderer<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        for tile in &self.tiles {
            target.draw_with_renderstates(tile, states);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tilemap::{TileMap, TileMapRenderer};

    #[test]
    fn test_tile_map_renderer_new() {
        let tile_map = TileMap::new((5, 5), 1);
        let renderer = TileMapRenderer::new(&tile_map);

        assert_eq!(renderer.tiles.len(), 25);
    }
}
