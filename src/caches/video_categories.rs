use std::collections::HashMap;
use std::sync::Mutex;
use fluffy::{db, model::Model};
use crate::models::{VideoCategories};

lazy_static! { 
    /// Caches all social frequency classification
    pub static ref VIDEO_CATEGORIES: Mutex<HashMap<usize, String>> = { 
        let rows = get_cache_items();
        Mutex::new(rows)
    };
}

/// Refresh the cache
pub fn refresh() { 
    let mut list = VIDEO_CATEGORIES.lock().unwrap();
    *list = get_cache_items();
}

/// Get all video classification
fn get_cache_items() -> HashMap<usize, String> { 
    let mut conn = db::get_conn();
    let query = query![
        fields => "id, name",
    ];
    let mut list = HashMap::new();
    let rows = VideoCategories::fetch_rows(&mut conn, &query, None);
    for r in rows { 
        let (id, name): (usize, String) = from_row!(r);
        list.insert(id, name);
    }
    list
}
