use fluffy::{DbRow, model::Model,};
use super::ModelBackend;
use serde_derive::{Serialize};
use std::collections::HashMap;
use crate::validations::Validator;

#[derive(Default, Debug, Serialize)]
pub struct Navs { 
    pub id: usize,
    pub name: String,
    pub url: String, 
    pub remark: String,
    pub seq: isize,
    pub is_blank: u32,
}

impl Model for Navs { 
    fn get_table_name() -> &'static str { "navs" }
}

impl ModelBackend for Navs { 

    type M = Self;

    get_fields!(Self, [
        name => String,
        url => String,
        is_blank => u32,
        remark => String,
        seq => isize,
    ]);

    fn validate(data: &HashMap<String, String>) -> Result<(), String> { 
        Validator::load(&data)
            .string_length("name", "The classification name must be between 2-20", 2, 20, true)
            .string_length("url", "The link address cannot be empty", 1, 200, true)
            .is_yes_no("is_blank", "You must choose whether it is an external link")
            .string_limit("remark", "Remarks cannot exceed 200", 200)
            .is_numeric("seq", "Sort must be numbers")
            .validate()
    }
}
