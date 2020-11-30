use retroland::texture;
use retroland::tilemap::{TileMap, TileMapRenderer};
use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::window::{Event, Style, VideoMode};
use std::convert::TryFrom;
use std::env;
use std::net::TcpStream;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Get server address
    let server_addr = args.get(1).map(|v| &v[..]).unwrap_or(&"127.0.0.1:4567");

    // Connect to the server
    let stream = TcpStream::connect(server_addr)
        .unwrap_or_else(|_| panic!("unable to connect to {}", server_addr));

    // Receive the tile map
    let tile_map = TileMap::try_from(stream).expect("unable to read tile map");
    println!("size x: {}, y: {}", tile_map.size().x, tile_map.size().y);

    let mut window = RenderWindow::new(
        VideoMode::desktop_mode(),
        "Retroland Client",
        Style::DEFAULT,
        &Default::default(),
    );

    let textures = texture::load("assets").expect("unable to load textures");

    // Create tile map renderer
    let viewport_size = (15, 15).into();
    let renderer = TileMapRenderer::new(
        &tile_map,
        window.size(),
        viewport_size,
        window.default_view().to_owned(),
        &textures,
        true,
    );

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if let Event::Closed = event {
                window.close();
            }
        }

        window.clear(Color::BLACK);
        window.draw(&renderer);
        window.display();
    }
}
