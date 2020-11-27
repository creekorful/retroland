use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::system::{Clock, Vector2f};
use sfml::window::mouse::Button;
use sfml::window::{Event, Key, Style};

use crate::tilemap::{TileMap, TileMapRenderer};
use std::convert::TryFrom;
use std::env;
use std::fs::File;

mod tilemap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let save_file = args.get(1);

    let mut window = RenderWindow::new(
        (1920, 1080),
        "Retroland",
        Style::DEFAULT,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    let mut tile_map = TileMap::new((30, 20), 1);
    // Try to load tile map from file
    if let Some(save_file) = save_file {
        let file = File::open(save_file);
        if let Ok(file) = file {
            tile_map = TileMap::try_from(file).unwrap();
        }
    }

    let mut viewport_size = (15, 15).into();
    let mut renderer = TileMapRenderer::new(
        &tile_map,
        window.size(),
        viewport_size,
        window.default_view().to_owned(),
    );

    let mut tile_id = 2 as u32;
    let mut delta_clock = Clock::default();
    while window.is_open() {
        let delta_time = delta_clock.restart();
        let move_factor = 4000.0 * delta_time.as_seconds();

        let mut offset = Vector2f::default();
        while let Some(event) = window.poll_event() {
            if let Event::Closed = event {
                window.close();
            }

            if let Event::KeyPressed { code, ctrl, .. } = event {
                match code {
                    // Zoom control
                    Key::Add => {
                        // prevent having a 0 viewport size (will cause crash)
                        if viewport_size.x != 1 {
                            viewport_size = (viewport_size.x - 1, viewport_size.y - 1).into();
                        }
                    }
                    Key::Subtract => {
                        viewport_size = (viewport_size.x + 1, viewport_size.y + 1).into();
                    }
                    // Tile selection control
                    Key::Num1 => {
                        tile_id = 1;
                    }
                    Key::Num2 => {
                        tile_id = 2;
                    }
                    Key::Num3 => {
                        tile_id = 3;
                    }
                    // Other controls
                    Key::S => {
                        if ctrl {
                            if let Some(save_file) = save_file {
                                let file = File::create(save_file).unwrap();
                                tile_map.write(&file).unwrap();
                            }
                            continue; // no further processing
                        }
                    }
                    _ => {}
                }

                // Re create the renderer with updated details
                if code == Key::Add || code == Key::Subtract {
                    renderer.update(&tile_map, window.size(), viewport_size);
                }
            }

            // Manage click event
            if Button::Left.is_pressed() {
                let world_pos = window.map_pixel_to_coords_current_view(window.mouse_position());
                if let Some(map_position) = renderer.get_tile_position(world_pos) {
                    tile_map.set_tile(map_position, 0, tile_id).unwrap();

                    // update the renderer
                    renderer.update(&tile_map, window.size(), viewport_size);
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
