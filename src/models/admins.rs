use fluffy::{DbRow, model::Model, random, utils};
use super::ModelBackend;
use serde_derive::{Serialize};
use crate::validations::Validator;
use std::collections::HashMap;

#[derive(Default, Debug, Serialize)]
pub struct Admins { 
    pub id: usize, //serial number
    pub name: String, //user name
    pub last_ip: String, //Last login IP
    pub state: u32, //State, whether it is available, 0: unavailable, 1: available
    pub login_count: u32, //Number of login
    pub last_login: u32, //last login time
    pub created: u32, //add time
    pub updated: u32, //Update time
    pub role_id: usize,
    pub seq: isize,
}

impl Model for Admins { 
    fn get_table_name() -> &'static str { "admins" }
}

impl ModelBackend for Admins { 

    type M = Self;

    get_fields!(Self, [
        name => String,
        last_ip => String,
        role_id => usize,
        state => u32,
        login_count => u32,
        last_login => u32,
        created => u32,
        updated => u32,
        seq => isize,
    ]);

    fn save_before(data: &mut HashMap<String, String>) { 
        if let Some(v) = data.get("password") {  //If you submit a password
            let secret = random::rand_str(32);
            let password = utils::get_password(&secret, v);
            data.insert("password".to_owned(), password);
            data.insert("secret".to_owned(), secret);
        }
    }

    fn validate(data: &HashMap<String, String>) -> Result<(), String> { 
        let mut vali = Validator::load(&data);
        let mut need_password = false;
        if let Some(v) = data.get("id") { 
            if v == "0" { 
                need_password = true
            } else if let Some(_) = data.get("password") { 
                need_password = true
            }
        } 

        println!("data = {:?}", data);
        if need_password {
            vali.is_password("password", "Must enter a password");
            vali.equal("password", "re_password", "The password you input must be consistent");
        }
        vali.is_username("name", "Please enter the user name in the correct format", true)
        .is_yes_no("state", "Incorrect state value")
        .is_numeric("seq", "Sort must be effective numbers")
        .validate()
    }

}
