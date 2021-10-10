use std::env;
use tracing_subscriber::{fmt::format::FmtSpan, prelude::*};
use std::collections::HashMap;

pub type RPCMap = HashMap<String, _>;

/// This is the service definition. It looks a lot like a trait definition.
/// It defines one RPC, hello, which takes one arg, name, and returns a String.
#[tarpc::service]
pub trait RPCService {
    async fn addr(request: RPCMap) -> RPCMap;
    async fn alert(request: RPCMap) -> RPCMap;
    async fn block(request: RPCMap) -> RPCMap;
    async fn checkorder(request: RPCMap) -> RPCMap;
    async fn checkpoint(request: RPCMap) -> RPCMap;
    async fn getaddr(request: RPCMap) -> RPCMap;
    async fn getblocks(request: RPCMap) -> RPCMap;
    async fn getdata(request: RPCMap) -> RPCMap;
    async fn headers(request: RPCMap) -> RPCMap;
    async fn inv(request: RPCMap) -> RPCMap;
    async fn mempool(request: RPCMap) -> RPCMap;
    async fn ping(request: RPCMap) -> RPCMap;
    async fn pong(request: RPCMap) -> RPCMap;
    async fn reply(request: RPCMap) -> RPCMap;
    async fn tx(request: RPCMap) -> RPCMap;
    async fn version(request: RPCMap) -> RPCMap;
    async fn verack(request: RPCMap) -> RPCMap;
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

