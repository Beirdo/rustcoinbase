use log::*;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::str::FromStr;
use super::peerdb::*;

#[derive(Clone)]
pub struct LocalAddressMap {
    addr_map: Arc<Mutex<HashMap<String, u64>>>,
}

impl LocalAddressMap {
    pub fn new() -> LocalAddressMap {
        LocalAddressMap { addr_map: Arc::new(Mutex::new(HashMap::new())) }
    }

    pub fn increment(&mut self, key: &MySocketAddr) -> bool {
       let key = String::from(key.to_string());
       let mut map = self.addr_map.lock().unwrap();
       if !((*map).contains_key(&key)) {
           return false;
       }

       let mut value = (*map).entry(key).or_insert(0);
       *value += 1;
       true
    }

    pub fn insert(&mut self, key: &MySocketAddr, value: u64) {
       let key = String::from(key.to_string());
       let mut map = self.addr_map.lock().unwrap();
       (*map).insert(key, value);
    }

    pub fn keys(self) -> Vec<MySocketAddr> {
       let mut map = self.addr_map.lock().unwrap();
       let mut vec = Vec::new();
       for key in (*map).keys() {
           vec.push(MySocketAddr(SocketAddr::from_str(&key).unwrap()));
       }
       vec
    }

    pub fn get(&mut self, key: &MySocketAddr) -> u64 {
       let key = String::from(key.to_string());
       let mut map = self.addr_map.lock().unwrap();
       let mut value: u64 = *(*map).get(&key).unwrap_or(&0);
       value
    }
}
