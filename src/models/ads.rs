use fluffy::{DbRow, model::Model,};
use super::ModelBackend;
use serde_derive::{Serialize};
use std::collections::HashMap;
use crate::validations::Validator;
use crate::caches::ads::{PAGE_IDS, POSITION_IDS};

#[derive(Default, Debug, Serialize)]
pub struct Ads { 
    pub id: usize,
    pub name: String,
    pub remark: String, 
    pub image: String,
    pub page_id: u32,
    pub position_id: u32,
    pub url: String,
    pub is_blank: u32,
    pub seq: isize,
}

impl Model for Ads { 
    fn get_table_name() -> &'static str { "ads" }
}

impl ModelBackend for Ads { 

    type M = Self;

    get_fields!(Self, [
        name => String,
        remark => String,
        image => String,
        page_id => u32,
        position_id => u32,
        url => String,
        is_blank => u32,
        seq => isize,
    ]);

    fn validate(data: &HashMap<String, String>) -> Result<(), String> { 
        Validator::load(&data)
            .string_length("name", "The name must be between 2-20", 2, 20, true)
            .string_limit("remark", "Remarks cannot exceed 200", 200)
            .string_limit("image", "The picture address cannot exceed 200 characters", 200)
            .in_range("page_id", "Must be within the range of available", &PAGE_IDS)
            .in_range("position_id", "The position must be within the range", &POSITION_IDS)
            .string_limit("url", "The length of the link address cannot exceed 200", 200)
            .is_yes_no("is_blank", "You must enter the option whether the external link")
            .is_numeric("seq", "Sort must be effective numbers")
            .validate()
    }
}
