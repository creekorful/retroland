use sfml::graphics::{
    Color, Drawable, RectangleShape, RenderStates, RenderTarget, Shape, Transformable, View,
};
use sfml::system::{SfBox, Vector2f, Vector2u};

use crate::tilemap::TileMap;
use std::ops::Sub;

/// Tile map renderer is used to render a tile map on the screen
pub struct TileMapRenderer<'s> {
    tiles: Vec<RectangleShape<'s>>,
    view: SfBox<View>,
    view_center: Vector2f,
    tile_size: f32,
    map_size: Vector2u,
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
            view_center: default_view.center(),
            view: default_view,
            tile_size,
            map_size: tile_map.size(),
        }
    }

    /// Move the renderer by given offset
    /// this will update the renderer inner view and 'move' the tile map
    pub fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        self.view.move_(offset)
    }

    /// Translate world position to tile position
    pub fn get_tile_position<O: Into<Vector2f>>(&self, world_pos: O) -> Option<Vector2u> {
        // Compute the offset between initial map view center and current one
        // this allows us to compute the 'real' tile position if the map has been moved
        let view_offset = Vector2f::new(
            self.view_center.x - self.view.center().x,
            self.view_center.y - self.view.center().y,
        );

        // Apply the offset to retrieve the world pos relative to the tile map
        let world_pos = world_pos.into();
        let world_pos = world_pos.sub(view_offset);

        let position = Vector2u::new(
            world_pos.x as u32 / self.tile_size as u32,
            world_pos.y as u32 / self.tile_size as u32,
        );
        if position.x >= self.map_size.x || position.y >= self.map_size.y {
            return None;
        }

        Some(position)
    }

    /// Return a reference to the actual renderer view
    pub fn view(&self) -> &SfBox<View> {
        &self.view
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
        assert_eq!(renderer.tile_size, 216.0); // We want a 5x5 viewport, therefore size will be 1080/5
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

    #[test]
    fn test_tile_map_renderer_get_tile_position() {
        let tile_map = TileMap::new((5, 5), 1);
        let mut renderer = TileMapRenderer::new(
            &tile_map,
            (1920, 1080),
            (5, 5),
            View::new((0.0, 0.0).into(), (10.0, 10.0).into()),
        );

        assert_eq!(renderer.get_tile_position((0.0, 0.0)), Some((0, 0).into()));
        assert_eq!(
            renderer.get_tile_position((210.0, 120.0)),
            Some((0, 0).into())
        );
        assert_eq!(
            renderer.get_tile_position((420.0, 210.0)),
            Some((1, 0).into())
        );
        assert_eq!(
            renderer.get_tile_position((-420.0, 210.0)),
            Some((0, 0).into())
        );
        assert_eq!(renderer.get_tile_position((12420.0, 210.0)), None);

        // Move the map and check if we are retrieving the 'real' tile position
        renderer.move_(Vector2f::new(400.0, 200.0));
        assert_eq!(
            renderer.get_tile_position((420.0, 210.0)),
            Some((3, 1).into())
        );
    }
}
