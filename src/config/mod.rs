use std::fs::File;
use std::io::prelude::*;
use serde_derive::Deserialize;
use std::env;

/// Get TOML related configuration
macro_rules! get_setting_from_toml { 
    ($struct: ident) => ({ 
        let result = $struct::default();
        let current_dir = if let Ok(v) = env::current_dir() { v } else { return result; };
        let current_path = if let Some(v) = current_dir.to_str() { v } else { return result; };
        let toml_file = format!("{}/setting.toml", current_path);
        match File::open(&toml_file) { 
            Ok(mut v) => { 
                let mut content = String::new();
                if let Ok(_) = v.read_to_string(&mut content) { 
                    if let Ok(t) = toml::from_str::<$struct>(&content) { t } else { result }
                } else { result }
            },
            Err(err) => { 
                println!("Reading files failed: {}", err);
                result
            }
        }
    })
}

/// Maximum number of mistakes in the login
pub const LOGIN_ERROR_MAX: usize = 1000;

/// Locking time after logging in
pub const LOGIN_LOCKED_TIME: usize = 3600;

/// Make uploaded picture type
pub const UPLOAD_IMAGE_TYPES: [&'static str; 6] = ["image/jpg", "image/png", "image/jpeg", "image/bmp", "image/gif", "image/webp"];

/// Binding host/port
#[derive(Deserialize, Default, Debug)]
pub struct App { 
    pub host: String,
    pub port: usize,
}

/// Database connection information
#[derive(Deserialize, Default, Debug)]
pub struct Database { 
    pub host: String,
    pub name: String,
    pub user: String,
    pub password: String,
    pub port: usize,
}

/// OSS configuration information
#[derive(Deserialize, Default, Debug)]
pub struct OSS { 
    pub access_key_id: String,
    pub access_key_secret: String,
    pub end_point: String,
    pub region: String,
    pub bucket: String,
}

/// System configuration information
#[derive(Deserialize, Default, Debug)]
pub struct Setting { 
    pub app: App,
    pub database: Database,
    pub oss: OSS,
}

lazy_static! { 
    pub static ref SETTING: Setting = get_setting_from_toml!(Setting);
    //pub static ref DB_INFO: Database = { dbg!(get_setting_from_toml!(Database)) };
    //pub static ref APP_INFO: App = { get_setting_from_toml!(App) };
    //pub static ref OSS_INFO: OSS = { get_setting_from_toml!(OSS) };
}


/// Get the database connection string
pub fn get_conn_string() -> String { 
    let setting = &*SETTING;
    let db = &setting.database;
    format!("mysql://{}:{}@{}:{}/{}", db.user, db.password, db.host, db.port, db.name)
}
