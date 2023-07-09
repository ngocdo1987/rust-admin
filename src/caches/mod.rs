use std::collections::HashMap;
use std::sync::Mutex;
use fluffy::{model::Db, db};
use crate::config;

/// State description
pub const STATES: [&'static str; 2] = ["Disable", "Enable"];

lazy_static! { 
    /// Database-table-field mapping relationship
    pub static ref TABLE_FIELDS: Mutex<HashMap<String, Vec<String>>> = {
        let mut conn = db::get_conn();
        let setting = &*config::SETTING;
        let info = &setting.database;
        let table_fields = Db::get_table_fields(&mut conn, &info.name);
        Mutex::new(table_fields)
    };
}

pub mod menus;
pub mod admin_roles;
pub mod ads;
pub mod video_categories;
pub mod video_tags;
pub mod video_authors;
