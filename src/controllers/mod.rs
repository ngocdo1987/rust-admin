use std::collections::HashMap;
use std::fmt::Debug;
use fluffy::{ tmpl::Tpl, response, model::Model, model::Db, data_set::DataSet, db, cond_builder::CondBuilder, datetime};
use crate::models::ModelBackend;
use actix_web::{HttpResponse, web::{Path, Form}, HttpRequest};
use crate::caches;
use serde::ser::{Serialize};
use actix_session::{Session};
use crate::common::Acl;
use percent_encoding::{percent_decode};

pub trait Controller { 
    
    /// Model
    type M: ModelBackend + Default + Serialize + Debug;
    
    /// Get the controller name
    fn get_controller_name() -> &'static str { 
        Self::M::get_table_name()
    }
    
    /// Get the query condition
    fn get_query_cond() -> Vec<(&'static str, &'static str)> { vec![] }

    /// Get the final query condition
    fn get_cond(queries: &HashMap<&str, &str>) -> CondBuilder { 
        let mut cond = CondBuilder::new();
        let conditions = Self::get_query_cond();
        for c in &conditions { 
            let field = c.0;
            let sign = c.1;
            if let Some(value) = queries.get(field) {
                let value_bytes = value.trim().as_bytes();
                let real_value = if let Ok(v) = percent_decode(value_bytes).decode_utf8() { v } else { continue; };
                if real_value == "" { 
                    continue;
                }
                match sign { 
                    "=" => { cond.eq(field, &real_value); },
                    "!=" => { cond.ne(field, &real_value); },
                    ">" => { cond.gt(field, &real_value); },
                    ">=" => { cond.gte(field, &real_value); },
                    "<" => { cond.lt(field, &real_value); },
                    "<=" => { cond.lte(field, &real_value); },
                    "%" => { cond.like(field, &real_value); },
                    _ => { }
                };
            }
            if sign == "[]" {  //Digital interval
                let key1 = format!("{}_start", field);
                let value1 = if let Some(v) = queries.get(key1.as_str()) { v.trim() }  else { continue; };
                let key2 = format!("{}_end", field);
                let value2 = if let Some(v) = queries.get(key2.as_str()) { v.trim() } else { continue; };
                if value1 == "" || value2 == "" { 
                    continue;
                }
                cond.between(field, &value1, &value2);
            }
            if sign == "[date]" {  //Date interval
                let key1 = format!("{}_start", field);
                let value1 = if let Some(v) = queries.get(key1.as_str()) { v.trim() } else { "" };
                if value1 != "" { 
                    let date_str = format!("{} 00:00:00", value1);
                    let timestamp = datetime::from_str(date_str.as_str()).timestamp();
                    cond.gt(field, &timestamp);
                }
                let key2 = format!("{}_end", field);
                let value2 = if let Some(v) = queries.get(key2.as_str()) { v.trim() } else { "" };
                if value2 != "" { 
                    let date_str = format!("{} 00:00:00", value2);
                    let timestamp = datetime::from_str(date_str.as_str()).timestamp();
                    cond.lte(field, &timestamp);
                }
            }
        }

        cond
    }
    
    /// Process additional recovery data
    fn index_after(_data: &mut tera::Context) {}
    
    /// Homepage
    fn index(request: HttpRequest, session:Session, tpl: Tpl) -> HttpResponse { 
        if !Acl::check_login(&session) || !Acl::check_auth(&request, &session) { 
            return response::redirect("/index/error");
        }
        let query_string = request.query_string();
        let queries = fluffy::request::get_queries(query_string);
        let query_cond = Self::get_cond(&queries);
        let cond = if query_cond.len() > 0 { Some(&query_cond) } else { None };
        let controller_name = Self::get_controller_name(); //Controller name
        let info = Self::M::get_records(&request, cond);
        let breads = &*caches::menus::BREADS.lock().unwrap();
        let bread_path = if let Some(v) = breads.get(&format!("/{}", controller_name)) { v } else { "" };
        let mut data = tmpl_data![
            "action_name" => &"index",
            "controller_name" => &controller_name,
            "records" => &info.records,
            "pager" => &info.pager,
            "bread_path" => &bread_path,
        ];
        let conds = Self::get_query_cond();
        for (key, sign) in &conds { 
            if sign == &"[]" || sign == &"[date]" {
                let key1 = format!("{}_start", key);
                let value1 = queries.get(key1.as_str()).unwrap_or(&"");
                data.insert(key1, &value1);
                let key2 = format!("{}_end", key);
                let value2 = queries.get(key2.as_str()).unwrap_or(&"");
                data.insert(key2, &value2);
                continue;
            }
            let value = queries.get(key).unwrap_or(&"");
            //if NUMBERS.is_match(value) {  //If it is a number, convert it to a number
            //    if let Ok(n) = dbg!(value.parse::<usize>()) { 
            //        data.insert(key.to_owned(), &n);
            //        continue;
            //    } 
            //}
            let value_bytes = value.as_bytes();
            let real_value = if let Ok(v) = percent_decode(value_bytes).decode_utf8() { v } else { continue; };
            data.insert(key.to_owned(), &real_value);
        }
        Self::index_after(&mut data);
        let view_file = &format!("{}/index.html", controller_name);
        render!(tpl, view_file, &data)
    }

    /// The additional data that needs to be displayed when processing editors
    fn edit_after(_data: &mut tera::Context) {}

    /// edit
    fn edit(request: HttpRequest, session: Session, info: Path<usize>, tpl: Tpl) -> HttpResponse { 
        if !Acl::check_login(&session) || !Acl::check_auth(&request, &session) { 
            return response::redirect("/index/error");
        }
        let controller_name = Self::get_controller_name(); //Controller name
        let id = info.into_inner();
        let is_update = id > 0;
        let row = if !is_update { Self::M::get_default() } else { 
            let fields = Self::M::get_fields();
            let query = query![fields => &fields, ];
            let cond = cond!["id" => id,];
            let mut conn = fluffy::db::get_conn();
            if let Some(r) = Self::M::fetch_row(&mut conn, &query, Some(&cond)) { 
                Self::M::get_record(r)
            } else { Self::M::get_default() }
        };
        let mut data = tmpl_data![
            "controller_name" => controller_name,
            "row" => &row,
            "id" => &id,
        ];
        Self::edit_after(&mut data);
        let view_file = &format!("{}/edit.html", controller_name);
        render!(tpl, view_file, &data)
    }

    /// edit
    fn save(request: HttpRequest, session: Session, info: Path<usize>, post: Form<HashMap<String, String>>) -> HttpResponse { 
        if !Acl::check_login(&session) || !Acl::check_auth(&request, &session) { 
            return response::error("Refuse to visit, unauthorized");
        }
        let id = info.into_inner();
        if id == 0 { Self::save_for_create(post) } else { Self::save_for_update(id, post) }
    }

    /// Add to
    fn save_for_create(post: Form<HashMap<String, String>>) -> HttpResponse { 
        let post_fields = post.into_inner();
        if let Err(message) = Self::M::validate(&post_fields) {  //If the error is tested
            return response::error(message);
        }
        let table_name = Self::M::get_table_name();
        let table_fields = caches::TABLE_FIELDS.lock().unwrap();
        let mut checked_fields = Db::check_fields(table_name, &table_fields, post_fields, false); //Data after testing
        Self::M::save_before(&mut checked_fields); //For detection before saving data
        let mut data = DataSet::create();
        for (k, v) in &checked_fields { 
            data.set(k, &v.trim());
        }
        let mut conn = db::get_conn();
        let id = Self::M::create(&mut conn, &data);
        if id > 0 { 
            Self::save_after();
            return response::ok();
        } 
        response::error("Increasing records failed")
    }
    
    /// Revise
    fn save_for_update(id: usize, post: Form<HashMap<String, String>>) -> HttpResponse { 
        let post_fields = post.into_inner();
        if let Err(message) = Self::M::validate(&post_fields) {  //If the error is tested
            return response::error(message);
        }
        let table_name = Self::M::get_table_name();
        let table_fields = caches::TABLE_FIELDS.lock().unwrap();
        let mut checked_fields = Db::check_fields(table_name, &table_fields, post_fields, true); //Data after testing
        Self::M::save_before(&mut checked_fields); //For detection before saving data
        let mut data = DataSet::update();
        for (k, v) in &checked_fields { 
            if k == "id" {  //Skip ID field
                continue;
            }
            data.set(k, &v.trim());
        }
        let mut conn = db::get_conn();
        let cond = cond![ "id" => &id, ];
        let id = Self::M::update(&mut conn, &data, &cond);
        if id > 0 { 
            Self::save_after();
            return response::ok();
        } 
        response::error("Modification record failed")
    }

    /// Treatment after saving
    fn save_after() { }
    
    /// delete
    fn delete(request: HttpRequest, session: Session, id_strings: Path<String>) -> HttpResponse { 
        if !Acl::check_login(&session) || !Acl::check_auth(&request, &session) { 
            return response::error("Refuse to visit, unauthorized");
        }
        let mut ids_string = String::new();
        for (index, value) in id_strings.split(",").enumerate() { 
            let _ = if let Ok(v) = value.parse::<usize>() { v } else { return response::error("Error parameter"); };
            if index > 0 { 
                ids_string.push_str(",");
            }
            ids_string.push_str(value);
        }
        let cond = cond![
            in_range => ["id" => &ids_string,],
        ];
        let mut conn = db::get_conn();
        let affected_rows = Self::M::delete(&mut conn, &cond);
        if affected_rows == 0 { response::error("No records") } else { 
            Self::delete_after();
            response::ok() 
        }
    }

    /// Treatment after deleting
    fn delete_after() { }
}

pub mod index;
pub mod admins;
pub mod admin_roles;
pub mod menus;
pub mod users;
pub mod video_categories;
pub mod videos;
pub mod video_replies;
pub mod video_tags;
pub mod user_levels;
pub mod watch_records;
pub mod ads;
pub mod navs;
pub mod configs;
pub mod video_authors;
