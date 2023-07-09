use fluffy::{model::{Model}, db};
use std::collections::HashMap;
use std::sync::Mutex;
use crate::models::{AdminRoles, Menus, MainMenu};
use regex::Regex;

lazy_static! { 
    pub static ref ADMIN_ROLES: Mutex<HashMap<usize, String>> = {
        let mut conn = db::get_conn();
        let query = query![
            fields => "id, name",
        ];
        let mut roles = HashMap::new();
        let rs = AdminRoles::fetch_rows(&mut conn, &query, None);
        for r in rs { 
            let (id, name): (usize, String) = from_row!(r);
            roles.insert(id, name);
        }
        Mutex::new(roles)
    };
}

lazy_static! { 
    pub static ref ROLE_MENUS: Mutex<HashMap<usize, Vec<MainMenu>>> = { 
        Mutex::new(Menus::get_role_menus())
    };
}

/// Whether to be allowed
pub fn allow_access(role_id: usize, url: &str) -> bool { 
    if url == "" { 
        return false;
    }
    let role_menus = &*ROLE_MENUS.lock().unwrap();
    if let Some(menus) = role_menus.get(&role_id) { 
        for menu in menus { 
            for sub in &menu.menus { 
                let regs = sub.url.split("|").collect::<Vec<&str>>();
                for reg in regs { 
                    let reg_url = &format!("^{}$", reg);
                    if let Ok(r) = Regex::new(reg_url) { 
                        if r.is_match(url) { 
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

fn refresh_roles() { 
    let mut conn = db::get_conn();
    let query = query![
        fields => "id, name",
    ];
    let mut roles = ADMIN_ROLES.lock().unwrap();
    (*roles).clear();
    let rs = AdminRoles::fetch_rows(&mut conn, &query, None);
    for r in rs { 
        let (id, name): (usize, String) = from_row!(r);
        (*roles).insert(id, name);
    }
}


/// Refresh the cache
pub fn refresh() { 
    refresh_roles(); //Refresh character information
    let mut role_menus = ROLE_MENUS.lock().unwrap();
    *role_menus = Menus::get_role_menus(); //Refresh the character/menu information
}
