use retroland::tilemap::TileMap;
use std::convert::TryFrom;
use std::env;
use std::net::TcpStream;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Get server address
    let server_addr = args.get(1).map(|v| &v[..]).unwrap_or(&"127.0.0.1:4567");

    // Connect to the server
    let stream =
        TcpStream::connect(server_addr).unwrap_or_else(|_| panic!("unable to connect to {}", server_addr));

    // Receive the tile map
    let tile_map = TileMap::try_from(stream).expect("unable to read tile map");
    println!("size x: {}, y: {}", tile_map.size().x, tile_map.size().y);
}
