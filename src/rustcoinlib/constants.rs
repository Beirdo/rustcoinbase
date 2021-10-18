use log::*;
use rand::Rng;

#[derive(Clone)]
pub struct Statics {
    pub min_peer_proto_version: i32,
    pub local_host_nonce: u64,
}

impl Statics {
    pub fn new() -> Statics {
        let mut rng = rand::thread_rng();
        let mut nonce = 0;
        while nonce <= 1 {
            nonce = rng.gen::<u64>();
        }

        info!("Local Host P2P Nonce: {:016X}", nonce);

        Statics {
            min_peer_proto_version: 60012,
            local_host_nonce: nonce,
        }
    }
}
