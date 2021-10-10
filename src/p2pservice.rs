use std::env;
use tracing_subscriber::{fmt::format::FmtSpan, prelude::*};
use std::collections::HashMap;

pub type P2PMap = HashMap<String, String>;

/// This is the service definition. It looks a lot like a trait definition.
/// It defines one RPC, hello, which takes one arg, name, and returns a String.
#[tarpc::service]
pub trait P2PService {
    async fn addr(request: P2PMap) -> P2PMap;
    async fn alert(request: P2PMap) -> P2PMap;
    async fn block(request: P2PMap) -> P2PMap;
    async fn checkorder(request: P2PMap) -> P2PMap;
    async fn checkpoint(request: P2PMap) -> P2PMap;
    async fn getaddr(request: P2PMap) -> P2PMap;
    async fn getblocks(request: P2PMap) -> P2PMap;
    async fn getdata(request: P2PMap) -> P2PMap;
    async fn headers(request: P2PMap) -> P2PMap;
    async fn inv(request: P2PMap) -> P2PMap;
    async fn mempool(request: P2PMap) -> P2PMap;
    async fn ping(request: P2PMap) -> P2PMap;
    async fn pong(request: P2PMap) -> P2PMap;
    async fn reply(request: P2PMap) -> P2PMap;
    async fn tx(request: P2PMap) -> P2PMap;
    async fn version(request: P2PMap) -> P2PMap;
    async fn verack(request: P2PMap) -> P2PMap;
}

/// Initializes an OpenTelemetry tracing subscriber with a Jaeger backend.
pub fn init_tracing(service_name: &str) -> anyhow::Result<()> {
    env::set_var("OTEL_BSP_MAX_EXPORT_BATCH_SIZE", "12");

    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name(service_name)
        .with_max_packet_size(2usize.pow(13))
        .install_batch(opentelemetry::runtime::Tokio)?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .try_init()?;

    Ok(())
}

