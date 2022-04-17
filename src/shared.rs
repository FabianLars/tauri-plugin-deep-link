use std::io::{BufRead, BufReader};

use interprocess::local_socket::{LocalSocketListener, LocalSocketStream};

fn handle_error(connection: std::io::Result<LocalSocketStream>) -> Option<LocalSocketStream> {
    connection
        .map_err(|error| eprintln!("Incoming connection failed: {}", error))
        .ok()
}

pub fn listen<F: FnMut(String) + Send + 'static>(identifier: String, mut handler: F) {
    std::thread::spawn(move || {
        let listener = LocalSocketListener::bind(identifier).expect("Can't create listener");

        for conn in listener.incoming().filter_map(handle_error) {
            let mut conn = BufReader::new(conn);
            let mut buffer = String::new();
            if let Err(io_err) = conn.read_line(&mut buffer) {
                log::error!("Error reading incoming connection: {}", io_err.to_string());
            };
            buffer.pop();

            handler(buffer);
        }
    });
}
