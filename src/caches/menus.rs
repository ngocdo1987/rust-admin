use std::collections::HashMap;
use std::sync::Mutex;
use fluffy::{db, model::Model};
use crate::models::Menus;

lazy_static! {
    /// Top menu
    pub static ref MAIN_MENUS: Mutex<HashMap<usize, String>> = Mutex::new(get_cache_items());
}

lazy_static! { 
    /// All menu information -mainly used to process bread crumbs
    pub static ref BREADS: Mutex<HashMap<String, String>> = Mutex::new(get_cache_breads());
}

/// Cache refresh
pub fn refresh() { 
    // main menu
    let mut main_menus = MAIN_MENUS.lock().unwrap();
    *main_menus = get_cache_items();
    // Submenu
    let mut breads = BREADS.lock().unwrap();
    *breads = get_cache_breads();
}

/// Get the cache item
fn get_cache_items() -> HashMap<usize, String> { 
    let fields = "id, name";
    let mut conn = db::get_conn();
    let cond = cond!["parent_id" => &"0", "is_show" => &"1", "state" => &"1",];
    let query = query![fields => &fields,];
    let rs = Menus::fetch_rows(&mut conn, &query, Some(&cond));
    let mut menus = HashMap::new();
    for r in rs { 
        let (id, name): (usize, String) = from_row!(r);
        menus.insert(id, name);
    }
    menus
}

// Get the cache-bread debris
fn get_cache_breads() -> HashMap<String, String> { 
    let menus = Menus::get_related();
    let mut breads: HashMap<String, String> = HashMap::new();
    for menu in &menus { 
        for sub in &menu.menus { 
            let bread = format!("<a href='#'>{}</a> <a href='#'><cite>{}</cite></a>", menu.name, sub.name);
            breads.insert(sub.url.to_owned(), bread);
        }
    }
    breads
}
