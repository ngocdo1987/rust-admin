use fluffy::{DbRow, model::Model, db,};
use super::ModelBackend;
use serde_derive::{Serialize};
use crate::validations::Validator;
use std::collections::HashMap;

#[derive(Default, Debug, Serialize)]
pub struct Menus { 
    pub id: usize, //serial number
    pub parent_id: usize, //Superior number
    pub name: String, //Menu name
    pub level_id: usize, //Menu level
    pub state: u32, //state
    pub is_blank: u32, //Whether to
    pub url: String, //link address
    pub seq: isize, //Sort
    pub is_show: u32, //Whether to display
}

#[derive(Default, Debug, Serialize)]
pub struct SubMenu { 
    pub id: usize,
    pub name: String,
    pub url: String,
    pub is_show: usize,
}

#[derive(Default, Debug, Serialize)]
pub struct MainMenu { 
    pub id: usize,
    pub name: String,
    pub menus: Vec<SubMenu>,
    pub is_show: usize,
}

impl Model for Menus { 
    fn get_table_name() -> &'static str { "menus" }
}

impl ModelBackend for Menus { 

    type M = Self;
    
    get_fields!(Self, [
        parent_id => usize,
        name => String,
        level_id => usize,
        state => u32,
        is_blank => u32,
        url => String,
        seq => isize,
        is_show => u32,
    ]);

    fn validate(data: &HashMap<String, String>) -> Result<(), String> { 
        Validator::load(&data)
            .is_numeric("parent_id", "The upper menu number must be effective numbers")
            .string_length("name", "The classification name must be between 2-20", 2, 20, true)
            .string_limit("url", "The length of the link address cannot exceed 200", 200)
            .is_yes_no("state", "Incorrect state value")
            .is_yes_no("is_blank", "Whether the external chain value is incorrect")
            .is_numeric("seq", "Sort must be effective numbers")
            .validate()
    }
}

impl Menus { 
    
    /// Get a related menu
    pub fn get_related() -> Vec<MainMenu> { 
        let mut main_menus = vec![];
        let mut conn = db::get_conn();
        let query = query![ fields => "id, name, url, is_show", ];
        let cond = cond![ "level_id" => "0", ];
        let rs_main = Menus::fetch_rows(&mut conn, &query, Some(&cond));
        for r_main in rs_main { 
            let (id, name, _, is_show): (usize, String, String, usize) = from_row!(r_main);
            let mut menus: Vec<SubMenu> = vec![];
            let cond_sub = cond!["parent_id" => &id, ];
            let rs_subs = Menus::fetch_rows(&mut conn, &query, Some(&cond_sub));
            for r_sub in rs_subs { 
                let (sub_id, sub_name, sub_url, sub_is_show): (usize, String, String, usize) = from_row!(r_sub);
                menus.push(SubMenu{ id: sub_id, name: sub_name, url: sub_url, is_show: sub_is_show });
            }
            main_menus.push(MainMenu{ id, name, menus, is_show, });
        }
        main_menus
    }
    
    /// Get the menu information according to the character number
    pub fn get_role_menus_by_id(role_id: usize, menu_ids: &String) -> Vec<MainMenu> { 
        let mut main_menus = vec![];
        let sql_subs = format!("SELECT parent_id FROM menus INNER JOIN admin_roles ON menus.id IN ({}) AND admin_roles.id = {}", menu_ids, role_id);
        let sql_main = format!("SELECT DISTINCT menus.id, menus.name, menus.is_show FROM menus WHERE id IN ({})", sql_subs);
        let mut conn = db::get_conn();
        let rows = Self::query(&mut conn, &sql_main);
        for r in rows { 
            let mut menus = vec![];
            let (main_id, main_name, main_is_show): (usize, String, usize) = from_row!(r);
            let sql_sub = format!("SELECT DISTINCT menus.id, menus.name, menus.url, menus.is_show FROM menus INNER JOIN admin_roles ON menus.id IN ({}) AND parent_id = {}", menu_ids, main_id);
            let subs = Self::query(&mut conn, sql_sub.as_str());
            for s in subs { 
                let (id, name, url, is_show): (usize, String, String, usize) = from_row!(s);
                menus.push(SubMenu{id, name, url, is_show});
            }
            main_menus.push(MainMenu{id: main_id, name: main_name, menus, is_show: main_is_show });
        }
        main_menus
    }
    
    /// Get all the character menu
    pub fn get_role_menus() -> HashMap<usize, Vec<MainMenu>> { 
        let sql = "SELECT id, menu_ids FROM admin_roles";
        let mut conn = db::get_conn();
        let rows = Self::query(&mut conn, &sql);
        let mut role_menus = HashMap::new();
        for r in rows { 
            let (role_id, menu_ids): (usize, String) = from_row!(r);
            let menus = Self::get_role_menus_by_id(role_id, &menu_ids);
            role_menus.insert(role_id, menus);
        }
        role_menus
    }
}
