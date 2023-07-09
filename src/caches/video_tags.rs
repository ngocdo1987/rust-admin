use std::collections::HashMap;
use std::sync::Mutex;
use fluffy::{db, model::Model};
use crate::models::{VideoTags};

lazy_static! { 
    /// Caches all label classification
    pub static ref VIDEO_TAGS: Mutex<HashMap<usize, String>> = { 
        let rows = get_cache_items();
        Mutex::new(rows)
    };
}

/// Refresh the cache
pub fn refresh() { 
    let mut list = VIDEO_TAGS.lock().unwrap();
    *list = get_cache_items();
}

/// Get all video classification
fn get_cache_items() -> HashMap<usize, String> { 
    let mut conn = db::get_conn();
    let query = query![
        fields => "id, name",
        limit => 100,
    ];
    let mut list = HashMap::new();
    let rows = VideoTags::fetch_rows(&mut conn, &query, None);
    for r in rows { 
        let (id, name): (usize, String) = from_row!(r);
        list.insert(id, name);
    }
    list
}


