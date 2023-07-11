use fluffy::{DbRow, model::Model,};
use super::ModelBackend;
use serde_derive::{Serialize};
use crate::validations::Validator;
use std::collections::HashMap;

#[derive(Default, Debug, Serialize)]
pub struct AdminRoles { 
    pub id: usize, //serial number
    pub name: String, //name
    pub remark: String, //Remark
    pub seq: isize, //
    pub menu_ids: String,
}

impl Model for AdminRoles { 
    fn get_table_name() -> &'static str { "admin_roles" }
}

impl ModelBackend for AdminRoles { 

    type M = Self;

    get_fields!(Self, [
        name => String,
        remark => String,
        seq => isize,
        menu_ids => String,
    ]);

    fn validate(data: &HashMap<String, String>) -> Result<(), String> { 
        Validator::load(&data)
            .string_length("name", "Role name's length must be between 2-20", 2, 20, true)
            .string_limit("remark", "Remark's length must be between 0-50", 50)
            .is_numeric("seq", "Sort must be effective numbers")
            .validate()
    }
}
