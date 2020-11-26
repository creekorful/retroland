mod tilemap;

use crate::tilemap::{TileMap, TileMapRenderer};
use sfml::graphics::{Color, RenderTarget, RenderWindow};
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
    let renderer = TileMapRenderer::new(&tile_map, window.size(), (5, 5).into());

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if let Event::Closed = event {
                window.close();
            }
        }

        window.clear(Color::BLUE);
        window.draw(&renderer);
        window.display();
    }
}
