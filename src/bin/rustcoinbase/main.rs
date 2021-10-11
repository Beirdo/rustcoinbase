use std::thread;

use rustcoinbase::rustcoinlib::peerdb::*;
use rustcoinbase::rustcoinlib::settings::*;

mod p2pserver;
use p2pserver::*;

fn main() {
    let settings = Settings::new("RustCoinBase").unwrap().clone();
    println!("{:?}", settings);

    let data_dir = String::clone(&settings.global.data_dir);
    let peerdb = init_peer_db(data_dir).expect("Couldn't initialize Peer DB");

    let mut handles = vec![];

    let handle = thread::spawn(move || {
        start_p2pserver(&settings, peerdb)
    });
    handles.push(handle);

    for handle in handles {
        if let Err(e) = handle.join() {
            println!("Couldn't join Thread!  Error {:?}", e);
            std::process::exit(1);
        }
    }
}
