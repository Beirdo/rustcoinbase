// Copyright 2018 Google LLC
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use clap::Clap;
use futures::{future, prelude::*};
use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};
use p2pservice::*;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};
use tarpc::{
    context,
    server::{self, incoming::Incoming, Channel},
};
use tokio_serde::formats::*;
//use tokio::time;
use std::collections::HashMap;

#[derive(Clap)]
struct Flags {
    /// Sets the port number to listen on.
    #[clap(long)]
    port: u16,
}

// This is the type that implements the generated World trait. It is the business logic
// and is used to start the server.
#[derive(Clone)]
struct P2PServer(SocketAddr);

#[tarpc::server]
impl P2PService for P2PServer {
    async fn version(self, _: context::Context, request: HashMap<String, String>) -> HashMap<String, String> {
        let message = format!("Version Request: {:?} from {}", request, self.0);
        println!("{}", message);
        let mut response = HashMap::new();
        response.insert(String::from("text"), String::from(message));
        response
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let flags = Flags::parse();
    init_tracing("Tarpc Example Server")?;

    let server_addr = (IpAddr::V4(Ipv4Addr::LOCALHOST), flags.port);
    tracing::info!("{:?}", server_addr);
    println!("{:?}", server_addr);

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
