use crate::tilemap::TileMap;
use sfml::graphics::{
    Color, Drawable, RectangleShape, RenderStates, RenderTarget, Shape, Transformable, View,
};
use sfml::system::Vector2u;

/// Tile map renderer is used to render a tile map on the screen
pub struct TileMapRenderer<'s> {
    tiles: Vec<RectangleShape<'s>>,
}

impl<'s> TileMapRenderer<'s> {
    pub fn new<T: Into<Vector2u>>(tile_map: &TileMap, screen_size: T, viewport_size: T) -> Self {
        let tile_map_size = tile_map.size();
        let screen_size = screen_size.into();

        let mut tiles = Vec::new();

        // Determinate tile size to fix them on whole screen
        let tile_width = screen_size.x / tile_map_size.x;
        let tile_height = screen_size.y / tile_map_size.y;
        let tile_size = if tile_width > tile_height {
            tile_width
        } else {
            tile_height
        } as f32;

        // At the moment only draw layer 0
        for y in 0..tile_map_size.y {
            for x in 0..tile_map_size.x {
                let mut tile = RectangleShape::new();
                tile.set_size((tile_size, tile_size));
                tile.set_position((x as f32 * tile_size, y as f32 * tile_size));
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
    use sfml::system::Vector2f;

    #[test]
    fn test_tile_map_renderer_new() {
        let tile_map = TileMap::new((5, 5), 1);
        let renderer = TileMapRenderer::new(&tile_map, (1920, 1080), (5, 5));

        assert_eq!(renderer.tiles.len(), 25);
        assert_eq!(
            renderer.tiles.get(0).unwrap().size(),
            Vector2f::new(384.0, 384.0)
        );
    }
}
