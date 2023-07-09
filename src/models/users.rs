use std::collections::HashMap;
use fluffy::{DbRow, model::Model,};
use super::ModelBackend;
use serde_derive::{Serialize};
use crate::validations::Validator;

#[derive(Default, Debug, Serialize)]
pub struct Users { 
    pub id: usize, //serial number
    pub name: String, //user name
    pub last_ip: String, //Last login IP
    pub state: u32, //State, whether it is available, 0: unavailable, 1: available
    pub login_count: u32, //Number of login
    pub last_login: u32, //last login time
    pub remark: String,
    pub created: u32, //add time
    pub updated: u32, //Update time
}

impl Model for Users { 
    fn get_table_name() -> &'static str { "users" }
}

impl ModelBackend for Users { 

    type M = Self;

    get_fields!(Self, [
        name => String,
        last_ip => String,
        state => u32,
        login_count => u32,
        last_login => u32,
        remark => String,
        created => u32,
        updated => u32,
    ]);

    fn validate(data: &HashMap<String, String>) -> Result<(), String> { 
        Validator::load(&data)
            .is_username("name", "Must be a user name", true)
            .validate()
    }
}
