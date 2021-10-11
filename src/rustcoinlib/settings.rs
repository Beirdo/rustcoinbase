extern crate directories;
use directories::ProjectDirs;

use std::env;
use config::{ConfigError, Config, File, Environment};
use serde_derive::Deserialize;
use std::path::Path;
use std::fs;
use std::net::IpAddr;
use std::sync::mpsc;

#[derive(Debug, Clone, Deserialize)]
pub struct Global {
    pub config_dir: String,
    pub data_dir: String,
    pub log_file: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct P2PSettings {
    pub port: u16,
    pub bind: IpAddr,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub global: Global,
    pub p2p: P2PSettings,
}

impl Settings {
    pub fn new(appname: &str, logqueue: mpsc::Sender<String>) -> Result<Self, ConfigError> {
        logqueue.send(format!("Loading settings for {}", appname)).unwrap();
        let mut s = Config::default();
        
        let config_dir = if let Some(proj_dirs) = ProjectDirs::from("net", "MyCryptoCoins", appname) {
            let config_dir = proj_dirs.config_dir().to_str().unwrap();
	    s.set("global.config_dir", config_dir)?;
            // Lin: /home/alice/.config/baseapp
            // Win: C:\Users\Alice\AppData\Roaming\MyCryptocoins\BaseApp\config
            // Mac: /Users/Alice/Library/Application Support/net.MyCryptoCoins.BaseApp
            String::from(config_dir)
        } else {
            String::from("config")
        };

        let data_dir = if let Some(proj_dirs) = ProjectDirs::from("net", "MyCryptoCoins", appname) {
            let data_dir = proj_dirs.data_dir().to_str().unwrap();
	    s.set("global.data_dir", data_dir)?;
            // Lin: /home/alice/.local/share/baseapp
            // Win: C:\Users\Alice\AppData\Roaming\MyCryptoCoins\BaseApp\data
            // Mac: /Users/Alice/Library/Application Support/net.MyCryptoCoins.Base-App
            String::from(data_dir)
        } else {
            String::from("data")
        };

	let log_file: String = String::from(Path::new(&data_dir).join(format!("{}.log", appname)).to_str().unwrap());
        s.set("global.log_file", log_file)?;

	// Create defaults - this sucks
        s.set("debug", false)?;
        s.set("p2p.port", 9911)?;
        s.set("p2p.bind", "0.0.0.0")?;

	if !Path::new(&config_dir).is_dir() {
            logqueue.send(format!("Dir doesn't exist: {}", config_dir)).unwrap();
            fs::create_dir_all(&config_dir).expect("Could not create config dir");
        }

	if !Path::new(&data_dir).is_dir() {
            logqueue.send(format!("Dir doesn't exist: {}", data_dir)).unwrap();
            fs::create_dir_all(&data_dir).expect("Could not create data dir");
        }

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name(&format!("{}/default", config_dir)).required(false))?;

        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("{}/{}", config_dir, env)).required(false))?;

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        s.merge(File::with_name(&format!("{}/local", config_dir)).required(false))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("app"))?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}
