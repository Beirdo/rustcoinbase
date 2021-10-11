use sled;
use serde_json::*;
use std::path::Path;

#[derive(Clone)]
pub struct PeerDatabase {
    db: sled::Db,
}

pub type PeerDBValue = Value;

pub fn init_peer_db(data_dir: String) -> sled::Result<PeerDatabase> {
    let path: String = String::from(Path::new(&data_dir).join("peerdb").to_str().unwrap());
    println!("{}", path);

    let db = sled::open(path)?;

    Ok(PeerDatabase { db })
}

impl PeerDatabase {
    pub fn write(&mut self, key: &str, value: PeerDBValue) -> sled::Result<()> {
	// value comes in as a JSON Value.  Convert to string, then to bytes
        let value = sled::IVec::from(value.to_string().as_bytes());
        self.db.insert(String::from(key), value)?;
        Ok(())
    }

    fn convert_value(in_value: Option<sled::IVec>) -> sled::Result<PeerDBValue> {
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

    pub fn read(&mut self, key: &str) -> sled::Result<PeerDBValue> {
        let value = self.db.get(String::from(key))?;
        PeerDatabase::convert_value(value)
    }

    pub fn contains(&mut self, key: &str) -> sled::Result<bool> {
        self.db.contains_key(String::from(key))
    }

    pub fn remove(&mut self, key: &str) -> sled::Result<PeerDBValue> {
        let value = self.db.remove(String::from(key))?;
        PeerDatabase::convert_value(value)
    }
}
