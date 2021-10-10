use std::thread;

use rustcoinbase::rustcoinlib::peerdb::*;

mod p2pserver;
use p2pserver::*;

fn main() {
    let peerdb = init_peer_db().expect("Couldn't initialize Peer DB");

    let mut handles = vec![];

    let handle = thread::spawn(move || {
        start_p2pserver(peerdb)
    });
    handles.push(handle);

    for handle in handles {
        if let Err(e) = handle.join() {
            println!("Couldn't join Thread!  Error {:?}", e);
            std::process::exit(1);
        }
    }
}
