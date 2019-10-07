use std::net::TcpListener;
use std::time::SystemTime;

fn main() {
    let listener =
        TcpListener::bind("0.0.0.0:8006").expect("Could not bind to socket");
    let mut conncount = 0;
    let mut start = None;

    // Loop over incoming connections so that we accept them, but don't assign
    // them to a variable so that they immediately drop and are closed.
    for _ in listener.incoming() {
        conncount += 1;

        // It's more useful to know the time since our first connection, not
        // the time since the program started.
        if start.is_none() {
            start = Some(SystemTime::now());
        }

        let elapsed = start
            .unwrap()
            .elapsed()
            .expect("Could not get elapsed time!");
        println!(
            "{} connections in {}.{} seconds",
            conncount,
            elapsed.as_secs(),
            elapsed.subsec_millis()
        );
    }
}
