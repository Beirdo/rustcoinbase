use clap::Clap;
use futures::{future, prelude::*};
use rustcoinbase::rustcoinlib::p2pservice::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tarpc::{
    context,
    server::{self, incoming::Incoming, Channel},
};
use tokio_serde::formats::*;
use serde_cbor::Value;

#[derive(Clap)]
struct Flags {
    /// Sets the port number to listen on.
    #[clap(long)]
    port: u16,
}

#[derive(Clone)]
struct P2PServer(SocketAddr);

#[tarpc::server]
impl P2PService for P2PServer {
    async fn addr(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("addr request: {:?} from {}", request, self.0);
        println!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn alert(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("alert request: {:?} from {}", request, self.0);
        println!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn block(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("block request: {:?} from {}", request, self.0);
        println!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn checkorder(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("checkorder request: {:?} from {}", request, self.0);
        println!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn checkpoint(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("checkpoint request: {:?} from {}", request, self.0);
        println!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn getaddr(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("getaddr request: {:?} from {}", request, self.0);
        println!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn getblocks(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("getblocks request: {:?} from {}", request, self.0);
        println!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn getdata(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("getdata request: {:?} from {}", request, self.0);
        println!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn headers(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("headers request: {:?} from {}", request, self.0);
        println!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn inv(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("inv request: {:?} from {}", request, self.0);
        println!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn mempool(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("mempool request: {:?} from {}", request, self.0);
        println!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn ping(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("ping request: {:?} from {}", request, self.0);
        println!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn pong(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("pong request: {:?} from {}", request, self.0);
        println!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn reply(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("reply request: {:?} from {}", request, self.0);
        println!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn tx(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("tx request: {:?} from {}", request, self.0);
        println!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn version(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("version request: {:?} from {}", request, self.0);
        println!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }

    async fn verack(self, _: context::Context, request: P2PMap) -> P2PMap {
        let message = format!("verack request: {:?} from {}", request, self.0);
        println!("{}", message);
        let mut response = P2PMap::new();
        response.insert(Value::from(String::from("text")),
                        Value::from(String::from(message)));
        response
    }
}

#[tokio::main]
pub async fn start_p2pserver() -> anyhow::Result<()> {
    let flags = Flags::parse();
    init_tracing("Tarpc Example Server")?;

    let server_addr = (IpAddr::V4(Ipv4Addr::LOCALHOST), flags.port);
    tracing::info!("{:?}", server_addr);
    println!("Starting P2P Service at {:?}", server_addr);

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
            let server = P2PServer(channel.transport().peer_addr().unwrap());
            channel.execute(server.serve())
        })
        // Max 10 channels.
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;

    Ok(())
}