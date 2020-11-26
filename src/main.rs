mod tilemap;

use crate::tilemap::{TileMap, TileMapRenderer};
use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::system::Vector2f;
use sfml::window::{Event, Style};

fn main() {
    let mut window = RenderWindow::new(
        (1920, 1080),
        "Retroland",
        Style::DEFAULT,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    let tile_map = TileMap::new((10, 10), 1);
    let mut renderer = TileMapRenderer::new(
        &tile_map,
        window.size(),
        (5, 5).into(),
        window.default_view().to_owned(),
    );

    let mut last_mouse_pos = Vector2f::default();
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if let Event::Closed = event {
                window.close();
            }

            // Manage mouse 'drag' event (to move on map)
            if let Event::MouseButtonPressed { x, y, button } = event {
                last_mouse_pos = window.map_pixel_to_coords_current_view((x, y).into());
            }
            if let Event::MouseButtonReleased { x, y, button } = event {
                // Compute the distance between last and now
                let distance = Vector2f::new(x as f32 - last_mouse_pos.x, y as f32 - last_mouse_pos.y);
                println!("distance x: {}, y: {}", distance.x, distance.y);
                renderer.move_(-Vector2f::new(distance.x, distance.y));

                last_mouse_pos = window.map_pixel_to_coords_current_view((x, y).into());
            }
        }

        window.clear(Color::BLACK);
        window.draw(&renderer);
        window.display();
    }
}
