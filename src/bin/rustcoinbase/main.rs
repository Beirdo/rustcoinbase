#[macro_use] extern crate log;
// use log::*;
use simplelog::*;
use std::thread;
use std::fs::OpenOptions;
use std::sync::mpsc;

use rustcoinbase::rustcoinlib::peerdb::*;
use rustcoinbase::rustcoinlib::settings::*;
use rustcoinbase::rustcoinlib::localaddress::*;

mod p2pserver;

use p2pserver::*;

fn main() {
    let appname: String = String::from("RustCoinBase");
    let (logtx, logrx) = mpsc::channel();
    let settings = Settings::new(&appname, logtx).unwrap().clone();

    let data_dir = String::clone(&settings.global.data_dir);
    let log_file = String::clone(&settings.global.log_file);

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            ConfigBuilder::new()
                .set_thread_level(LevelFilter::Debug)
                .set_target_level(LevelFilter::Debug)
                .set_location_level(LevelFilter::Debug)
                .set_thread_mode(ThreadLogMode::Both)
                .build(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            ConfigBuilder::new()
                .set_thread_level(LevelFilter::Debug)
                .set_target_level(LevelFilter::Debug)
                .set_location_level(LevelFilter::Debug)
                .set_thread_mode(ThreadLogMode::Both)
                .build(),
            OpenOptions::new().append(true).open(log_file).unwrap(),
        ),
    ]).unwrap();

    info!("Starting {}", appname);

    for message in logrx {
        info!("{}", message);
    }

    info!("{:?}", settings);

    let peerdb = init_peer_db(data_dir).expect("Couldn't initialize Peer DB");
    let mut localaddresses = LocalAddressMap::new();

    let mut handles = vec![];

    let handle = thread::spawn(move || {
        start_p2pserver(&settings, peerdb, &localaddresses)
    });
    handles.push(handle);

    for handle in handles {
        if let Err(e) = handle.join() {
            error!("Couldn't join Thread!  Error {:?}", e);
            std::process::exit(1);
        }
    }
}
