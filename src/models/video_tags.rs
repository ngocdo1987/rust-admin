use fluffy::{DbRow, model::Model,};
use super::ModelBackend;
use serde_derive::{Serialize};
use std::collections::HashMap;
use crate::validations::Validator;

#[derive(Default, Debug, Serialize)]
pub struct VideoTags { 
    pub id: usize,
    pub name: String,
    pub remark: String, 
    pub seq: isize,
}

impl Model for VideoTags { 
    fn get_table_name() -> &'static str { "video_tags" }
}

impl ModelBackend for VideoTags { 

    type M = Self;

    get_fields!(Self, [
        name => String,
        remark => String,
        seq => isize,
    ]);

    fn validate(data: &HashMap<String, String>) -> Result<(), String> { 
        Validator::load(&data)
            .string_length("name", "The name must be between 2-20", 2, 20, true)
            .string_limit("remark", "The remark length must be between 0-50", 50)
            .is_numeric("seq", "Sort must be effective numbers")
            .validate()
    }
}
