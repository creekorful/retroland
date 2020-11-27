use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::system::{Clock, Vector2f};
use sfml::window::{Event, Key, Style};

use crate::tilemap::{TileMap, TileMapRenderer};

mod tilemap;

fn main() {
    let mut window = RenderWindow::new(
        (1920, 1080),
        "Retroland",
        Style::DEFAULT,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    let tile_map = TileMap::new((30, 20), 1);

    let mut viewport_size = (15, 15).into();
    let mut renderer = TileMapRenderer::new(
        &tile_map,
        window.size(),
        viewport_size,
        window.default_view().to_owned(),
    );

    let mut delta_clock = Clock::default();
    while window.is_open() {
        let delta_time = delta_clock.restart();
        let move_factor = 4000.0 * delta_time.as_seconds();

        let mut offset = Vector2f::default();
        while let Some(event) = window.poll_event() {
            if let Event::Closed = event {
                window.close();
            }

            // Zoom control
            if let Event::KeyPressed { code, .. } = event {
                match code {
                    Key::Add => {
                        // prevent having a 0 viewport size (will cause crash)
                        if viewport_size.x != 1 {
                            viewport_size = (viewport_size.x - 1, viewport_size.y - 1).into();
                        }
                    }
                    Key::Subtract => {
                        viewport_size = (viewport_size.x + 1, viewport_size.y + 1).into();
                    }
                    _ => {}
                }

                // Re create the renderer with updated details
                // TODO: update to prevent overhead?
                if code == Key::Add || code == Key::Subtract {
                    renderer = TileMapRenderer::new(
                        &tile_map,
                        window.size(),
                        viewport_size,
                        window.view().to_owned(),
                    );
                }
            }

            // Not using key pressed event cause we need to be notified
            // when the key is hold down
            if Key::Z.is_pressed() {
                offset.y = -move_factor;
            }
            if Key::Q.is_pressed() {
                offset.x = -move_factor;
            }
            if Key::S.is_pressed() {
                offset.y = move_factor;
            }
            if Key::D.is_pressed() {
                offset.x = move_factor;
            }
        }
        renderer.move_(offset);

        window.clear(Color::BLACK);
        window.draw(&renderer);
        window.display();
    }
}
