mod inventory;

use sfml::graphics::{Color, IntRect, RenderTarget, RenderWindow, Texture};
use sfml::system::{Clock, SfBox, Vector2f};
use sfml::window::mouse::Button;
use sfml::window::{Event, Key, Style, VideoMode};

use crate::inventory::Inventory;
use retroland::tilemap::{TileMap, TileMapRenderer};
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;

fn load_textures<P: AsRef<Path>>(
    assets_dir: P,
) -> Result<BTreeMap<u32, SfBox<Texture>>, Box<dyn Error>> {
    let mut textures = BTreeMap::new();

    // Load from grass.png
    for i in 0..5 {
        textures.insert(
            i as u32,
            Texture::from_file_with_rect(
                &format!("{}/grass.png", assets_dir.as_ref().display()),
                &IntRect::new(i * 16, 0, 16, 16),
            )
            .ok_or_else(|| "unable to load texture".to_string())?,
        );
    }

    Ok(textures)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let save_file = args.get(1);
    let map_width = args.get(2).map(|v| v.parse().unwrap()).unwrap_or(30);
    let map_height = args.get(3).map(|v| v.parse().unwrap()).unwrap_or(20);

    let mut window = RenderWindow::new(
        VideoMode::desktop_mode(),
        "Retroland Editor",
        Style::DEFAULT,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    let mut tile_map = TileMap::new((map_width, map_height), 1);
    // Try to load tile map from file
    if let Some(save_file) = save_file {
        let file = File::open(save_file);
        if let Ok(file) = file {
            tile_map = TileMap::try_from(file).unwrap();
        }
    }

    // Load textures
    let textures = load_textures("assets").expect("unable to load textures");

    // Create inventory
    let mut show_inventory = false;
    let inventory = Inventory::new(window.size(), &textures);

    // Create tile map
    let mut viewport_size = (15, 15).into();
    let mut renderer = TileMapRenderer::new(
        &tile_map,
        window.size(),
        viewport_size,
        window.default_view().to_owned(),
        &textures,
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
                    Key::E => {
                        show_inventory = !show_inventory;
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

                if show_inventory {
                    if let Some(item_id) = inventory.get_item_id(world_pos) {
                        tile_id = item_id;
                        show_inventory = false; // hide inventory if an item has been selected
                    }
                } else if let Some(map_position) = renderer.get_tile_position(world_pos) {
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
        if show_inventory {
            window.draw(&inventory);
        }
        window.display();
    }
}
