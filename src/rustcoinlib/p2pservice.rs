use std::collections::BTreeMap;
use serde_cbor::Value;
use serde_cbor::{to_vec, from_slice};
use serde_derive::{Serialize, Deserialize};

pub type P2PMap = BTreeMap<Value, Value>;

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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct P2PVersionRequest {
    #[serde(default)]
    pub version: i32,
    #[serde(default)]
    pub services: u64,
    #[serde(default)]
    pub time: u64,
    #[serde(default)]
    pub addr_me: String,
    #[serde(default)]
    pub addr_from: String,
    #[serde(default)]
    pub nonce: u64,
    #[serde(default)]
    pub sub_version: String,
    #[serde(default)]
    pub height: u64,
}

impl P2PVersionRequest {
    pub fn from(map: P2PMap) -> P2PVersionRequest {
        let cbor_raw = to_vec(&map).unwrap();
        let request: P2PVersionRequest = from_slice(&cbor_raw).unwrap();
        request
    }
}
