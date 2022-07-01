use std::env;

use pong::{server::start_server, client::start_client};

fn main() {
    // Collect command arguments
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--server".to_string()) {
        start_server();
    } else {
        start_client();
    }
}
