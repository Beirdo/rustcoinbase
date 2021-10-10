// Copyright 2018 Google LLC
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use clap::Clap;
use rustcoinbase::rustcoinlib::p2pservice::*;
use std::{net::SocketAddr, time::Duration};
use tarpc::{client, context};
use tokio_serde::formats::*;
use tokio::time::sleep;
use serde_cbor::Value;

#[derive(Clap)]
struct Flags {
    /// Sets the server address to connect to.
    #[clap(long)]
    server_addr: SocketAddr,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let flags = Flags::parse();
    init_tracing("Tarpc Example Client")?;

    let transport = tarpc::serde_transport::tcp::connect(flags.server_addr, Cbor::default);

    // WorldClient is generated by the service attribute. It has a constructor `new` that takes a
    // config and any Transport as input.
    let client = P2PServiceClient::new(client::Config::default(), transport.await?).spawn();

    let mut request = P2PMap::new();
    request.insert(Value::from(String::from("biteme")),
                   Value::from(String::from("dude")));
    request.insert(Value::from(String::from("eatme")),
                   Value::from(30));
    let version = client.addr(context::current(), request).await?;

    tracing::info!("{:?}", version);
    println!("{:?}", version);

    // Let the background span processor finish.
    sleep(Duration::from_micros(1000000)).await;
    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
