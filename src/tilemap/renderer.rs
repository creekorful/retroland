use sfml::graphics::{
    Color, Drawable, RectangleShape, RenderStates, RenderTarget, Shape, Transformable, View,
};
use sfml::system::{SfBox, Vector2f, Vector2u};

use crate::tilemap::TileMap;

/// Tile map renderer is used to render a tile map on the screen
pub struct TileMapRenderer<'s> {
    tiles: Vec<RectangleShape<'s>>,
    view: SfBox<View>,
}

impl<'s> TileMapRenderer<'s> {
    /// Create a new renderer using given tile map & display parameters
    ///
    /// # Arguments
    /// - tile_map: the inner tile map details
    /// - screen_size: the screen size in pixel
    /// - viewport_size: the expected viewport size (will affect number of tiles displayed on screen)
    /// - default_view: the default view to apply
    pub fn new<T: Into<Vector2u>>(
        tile_map: &TileMap,
        screen_size: T,
        viewport_size: T,
        default_view: SfBox<View>,
    ) -> Self {
        let tile_map_size = tile_map.size();
        let screen_size = screen_size.into();
        let viewport_size = viewport_size.into();

        let mut tiles = Vec::with_capacity((tile_map_size.x * tile_map_size.y) as usize);

        // Determinate tile size to fix them on whole screen
        // this algorithm will try to display at least the expected viewport size
        // this means that they **may** be more tiles displayed, depending on screen resolution
        let tile_width = screen_size.x / viewport_size.x;
        let tile_height = screen_size.y / viewport_size.y;
        let tile_size = if tile_width < tile_height {
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

        TileMapRenderer {
            tiles,
            view: default_view,
        }
    }

    /// Move the renderer by given offset
    /// this will update the renderer inner view and 'move' the tile map
    pub fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        self.view.move_(offset)
    }
}

impl<'s> Drawable for TileMapRenderer<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.set_view(&self.view);
        for tile in &self.tiles {
            target.draw_with_renderstates(tile, states);
        }
        target.set_view(&target.default_view().to_owned());
    }
}

#[cfg(test)]
mod tests {
    use sfml::graphics::View;
    use sfml::system::Vector2f;

    use crate::tilemap::{TileMap, TileMapRenderer};

    #[test]
    fn test_tile_map_renderer_new() {
        let tile_map = TileMap::new((5, 5), 1);
        let renderer = TileMapRenderer::new(
            &tile_map,
            (1920, 1080),
            (5, 5),
            View::new((0.0, 0.0).into(), (10.0, 10.0).into()),
        );

        assert_eq!(renderer.tiles.len(), 25);
        assert_eq!(
            renderer.tiles.get(0).unwrap().size(),
            (216.0, 216.0).into() // We want a 5x5 viewport, therefore size will be 1080/5
        );
    }

    #[test]
    fn test_tile_map_renderer_move() {
        let tile_map = TileMap::new((5, 5), 1);
        let mut renderer = TileMapRenderer::new(
            &tile_map,
            (1920, 1080),
            (5, 5),
            View::new((0.0, 0.0).into(), (10.0, 10.0).into()),
        );

        assert_eq!(renderer.view.size(), (10.0, 10.0).into());
        assert_eq!(renderer.view.center(), (0.0, 0.0).into());

        renderer.move_(Vector2f::new(10.0, 0.0));

        assert_eq!(renderer.view.center(), (10.0, 0.0).into());
    }
}
