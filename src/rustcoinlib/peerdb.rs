use log::*;
use sled;
use serde_json::*;
use std::path::Path;
use serde_derive::{Serialize, Deserialize};
use std::net::{SocketAddr, IpAddr};
use std::fmt;
use cidr::Ipv4Cidr;

#[derive(Clone)]
pub struct PeerDatabase {
    db: sled::Db,
}

pub type PeerDBValue = Value;

pub fn init_peer_db(data_dir: String) -> sled::Result<PeerDatabase> {
    let path: String = String::from(Path::new(&data_dir).join("peerdb").to_str().unwrap());
    info!("Peer Database at {}", path);

    let db = sled::open(path)?;

    Ok(PeerDatabase { db })
}

impl PeerDatabase {
    pub fn write_raw(&mut self, key: &str, value: PeerDBValue) -> sled::Result<()> {
	// value comes in as a JSON Value.  Convert to string, then to bytes
        let value = sled::IVec::from(value.to_string().as_bytes());
        self.db.insert(String::from(key), value)?;
        Ok(())
    }

    pub fn write(&mut self, key: &str, value: PeerDBRecord) -> sled::Result<()> {
	// value comes in as a JSON Value.  Convert to string, then to bytes
        let value = sled::IVec::from(value.to_string().as_bytes());
        self.db.insert(String::from(key), value)?;
        Ok(())
    }

    fn convert_value(in_value: Option<sled::IVec>) -> sled::Result<PeerDBRecord> {
	// Value comes back as bytes, unravel to a JSON string, then back to
        // JSON value

	// Make it an vec with default that maps to {}
        let value = in_value.unwrap_or(sled::IVec::from(vec![173, 175])).to_vec();
        // convert that to a UTF8 string
        let value = String::from_utf8(value).unwrap_or(String::from("{}"));
        // and finally from a String back to a JSON Value
        let value: PeerDBRecord = from_str(&value).unwrap();
        Ok(value)
    }

    fn convert_value_raw(in_value: Option<sled::IVec>) -> sled::Result<PeerDBValue> {
	// Value comes back as bytes, unravel to a JSON string, then back to
        // JSON value

	// Make it an vec with default that maps to {}
        let value = in_value.unwrap_or(sled::IVec::from(vec![173, 175])).to_vec();
        // convert that to a UTF8 string
        let value = String::from_utf8(value).unwrap_or(String::from("{}"));
        // and finally from a String back to a JSON Value
        let value: PeerDBValue = from_str(&value).unwrap();
        Ok(value)
    }

    pub fn read_raw(&mut self, key: &str) -> sled::Result<PeerDBValue> {
        let value = self.db.get(String::from(key))?;
        PeerDatabase::convert_value_raw(value)
    }

    pub fn read(&mut self, key: &str) -> sled::Result<PeerDBRecord> {
        let value = self.db.get(String::from(key))?;
        PeerDatabase::convert_value(value)
    }

    pub fn contains(&mut self, key: &str) -> sled::Result<bool> {
        self.db.contains_key(String::from(key))
    }

    pub fn remove(&mut self, key: &str) -> sled::Result<PeerDBRecord> {
        let value = self.db.remove(String::from(key))?;
        PeerDatabase::convert_value(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MySocketAddr(pub SocketAddr);
impl Default for MySocketAddr {
    fn default() -> Self {
        MySocketAddr(SocketAddr::new("0.0.0.0".parse().unwrap(), 0))
    }
}

impl fmt::Display for MySocketAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl MySocketAddr {
    pub fn is_routable(&self) -> bool {
        let ip = (*self).0.ip();
        info!("IP: {:?}", ip);

        match ip {
            IpAddr::V4(ipv4) => {
                /* handle IPv4 */
	        let bad_cidrs = [
                    Ipv4Cidr::new("0.0.0.0".parse().unwrap(), 0),
                    Ipv4Cidr::new("255.255.255.255".parse().unwrap(), 32), 
                    // !RFC1918 - 10/8, 172.16/12, 192.168/16
                    Ipv4Cidr::new("10.0.0.0".parse().unwrap(), 8),
                    Ipv4Cidr::new("172.16.0.0".parse().unwrap(), 12),
                    Ipv4Cidr::new("192.168.0.0".parse().unwrap(), 16),
                    // !RFC3927 - 169.254/16
                    Ipv4Cidr::new("169.254.0.0".parse().unwrap(), 16),
                    // !Local - 127/8
                    Ipv4Cidr::new("127.0.0.0".parse().unwrap(), 8),
                ];

                for bad_block in bad_cidrs {
                    if bad_block.unwrap().contains(&ipv4) {
                        return false;
                    }
                }
            }
            IpAddr::V6(_ipv6) => { return false; }
        }

        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerDBRecord {
    #[serde(default)]
    pub services: u64,
    #[serde(default)]
    pub last_send: i64,
    #[serde(default)]
    pub last_recv: i64,
    #[serde(default)]
    pub last_send_empty: i64,
    #[serde(default)]
    pub time_connected: i64,
    #[serde(default)]
    pub addr_remote: MySocketAddr,
    #[serde(default)]
    pub addr_name: String,
    #[serde(default)]
    pub addr_local: MySocketAddr,
    #[serde(default)]
    pub version: i32,
    #[serde(default)]
    pub sub_version: String,
    #[serde(default)]
    pub height: u64,
    #[serde(default)]
    pub one_shot: bool,
    #[serde(default)]
    pub client: bool,
    #[serde(default)]
    pub inbound: bool,
    #[serde(default)]
    pub network_node: bool,
    #[serde(default)]
    pub successfully_connected: bool,
    #[serde(default)]
    pub disconnect: bool,
    #[serde(default)]
    pub grant_outbound: bool,
    #[serde(default)]
    pub ref_count: i32,
    #[serde(default)]
    pub misbehaving: i32,
}

impl fmt::Display for PeerDBRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

