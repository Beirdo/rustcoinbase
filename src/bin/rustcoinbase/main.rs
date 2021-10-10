use std::thread;

mod p2pserver;
use p2pserver::*;

fn main() {
    let p2pthread = thread::spawn(|| start_p2pserver());

    p2pthread.join().unwrap();
}
