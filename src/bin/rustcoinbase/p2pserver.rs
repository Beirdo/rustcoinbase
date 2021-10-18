use log::*;
use futures::{future, prelude::*};
use rustcoinbase::rustcoinlib::p2pservice::*;
use rustcoinbase::rustcoinlib::peerdb::*;
use rustcoinbase::rustcoinlib::settings::*;
use rustcoinbase::rustcoinlib::constants::*;
use rustcoinbase::rustcoinlib::localaddress::*;
use std::net::{SocketAddr, IpAddr};
use tarpc::{
    context,
    server::{self, incoming::Incoming, Channel},
};
use tokio_serde::formats::*;
use serde_cbor::Value;
use serde_json;

#[derive(Clone)]
struct P2PServer(SocketAddr, SocketAddr, PeerDatabase, Statics, LocalAddressMap);

#[tarpc::server]
impl P2PService for P2PServer {
    async fn addr(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("addr request: {:?} from {}", request, self.0);

        let remote_addr = self.0;
        let _my_addr = self.1;
        let mut peerdb = self.2;
        let _constants = self.3;
        let key = String::from(remote_addr.to_string());

        let value: PeerDBValue = serde_json::json!({
            "test": "blah"
        });
        peerdb.write_raw(&key, value).unwrap();
        //let result = peerdb.read(&key).expect("Couldn't read back from db");
        info!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn alert(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("alert request: {:?} from {}", request, self.0);
        info!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn block(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("block request: {:?} from {}", request, self.0);
        info!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn checkorder(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("checkorder request: {:?} from {}", request, self.0);
        info!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn checkpoint(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("checkpoint request: {:?} from {}", request, self.0);
        info!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn getaddr(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("getaddr request: {:?} from {}", request, self.0);
        info!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn getblocks(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("getblocks request: {:?} from {}", request, self.0);
        info!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn getdata(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("getdata request: {:?} from {}", request, self.0);
        info!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn headers(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("headers request: {:?} from {}", request, self.0);
        info!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn inv(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("inv request: {:?} from {}", request, self.0);
        info!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn mempool(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("mempool request: {:?} from {}", request, self.0);
        info!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn ping(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("ping request: {:?} from {}", request, self.0);
        info!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn pong(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("pong request: {:?} from {}", request, self.0);
        info!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn reply(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("reply request: {:?} from {}", request, self.0);
        info!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn tx(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("tx request: {:?} from {}", request, self.0);
        info!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn version(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("version request: {:?} from {}", request, self.0);
        info!("{}", message);

        let request: P2PVersionRequest = P2PVersionRequest::from(request);
        info!("{:?}", request);

        let remote_addr = self.0;
        let my_addr = self.1;
        let mut peerdb = self.2;
        let constants = self.3;
        let mut localaddress = self.4;

        let key = String::from(remote_addr.to_string());
        let mut peer = peerdb.read(&key).expect("Couldn't read back from db");

        info!("{:?}", peer);

        let mut response = P2PMap::new();

	if peer.version != 0 {
            peer.misbehaving += 1;
            return response;
        }

	peer.version = request.version;
        peer.services = request.services;
        peer.addr_remote = MySocketAddr(remote_addr);

        let _time = request.time;
        let addr_me: IpAddr = request.addr_me.parse().unwrap();
        let sa = MySocketAddr(SocketAddr::new(addr_me, my_addr.port()));

        if peer.inbound && sa.is_routable() {
            peer.addr_local = sa.clone();
            if localaddress.increment(&sa) {
                // Readvertise with new values
            }
        }

        info!("Peer: {:?}", peer);

	if peer.version < constants.min_peer_proto_version {
            error!("Peer {:?} using obsolete version {}; disconnecting", remote_addr, peer.version);
            peer.disconnect = true;
            peerdb.write(&key, peer).unwrap();
            return response;
        }

        let nonce = request.nonce;
        if nonce == constants.local_host_nonce {
            error!("Connected to self from {}, disconnecting", remote_addr);
            peer.disconnect = true;
            return response;
        }
        
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn verack(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("verack request: {:?} from {}", request, self.0);
        info!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }
}

#[tokio::main]
pub async fn start_p2pserver(settings: &Settings, peerdb: PeerDatabase, localaddresses: &LocalAddressMap ) -> anyhow::Result<()> {
    let constants: Statics = Statics::new();
    let mut local_addr_map = localaddresses;
    let server_ip = settings.p2p.bind;
    let server_port = settings.p2p.port;
    let server_addr = (server_ip, server_port);
    info!("Starting P2P Service at {:?}", server_addr);

    // JSON transport is provided by the json_transport tarpc module. It makes it easy
    // to start up a serde-powered json serialization strategy over TCP.
    let mut listener = tarpc::serde_transport::tcp::listen(&server_addr, Cbor::default).await?;
    listener.config_mut().max_frame_length(usize::MAX);
    listener
        // Ignore accept errors.
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        // Limit channels to 1 per IP.
        .max_channels_per_key(1, |t| t.transport().peer_addr().unwrap().ip())
        // serve is generated by the service attribute. It takes as input any type implementing
        // the generated World trait.
        .map(|channel| {
            let server = P2PServer(channel.transport().peer_addr().unwrap(),
                                   channel.transport().local_addr().unwrap(),
                                   peerdb.clone(), constants.clone(),
                                   local_addr_map.clone());
            channel.execute(server.serve())
        })
        // Max 10 channels.
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;

    Ok(())
}
