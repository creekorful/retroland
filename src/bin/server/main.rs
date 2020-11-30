use retroland::tilemap::TileMap;
use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::net::TcpListener;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Open tile map
    let save_file = args.get(1).expect("missing required save file");
    println!("serving map `{}`", save_file);

    let save_file = File::open(save_file).expect("unable to open save file");
    let tile_map = TileMap::try_from(save_file).expect("unable to open save file");

    let address = "0.0.0.0:4567";
    let listener =
        TcpListener::bind(address).unwrap_or_else(|_| panic!("unable to listen to: {}", address));
    println!("listening on `{}`", address);

    for stream in listener.incoming() {
        let stream = stream.expect("unable to acquire stream");

        println!("new client {} is connected", stream.local_addr().unwrap());

        // Send the tile map to the client
        tile_map
            .write(stream)
            .expect("unable to send tile map to client");
    }
}
