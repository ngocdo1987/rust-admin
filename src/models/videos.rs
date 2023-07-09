use fluffy::{DbRow, model::Model,};
use super::ModelBackend;
use serde_derive::{Serialize};
use crate::validations::Validator;
use std::collections::HashMap;

#[derive(Default, Debug, Serialize)]
pub struct Videos { 
    pub id: usize,
    pub title: String,
    pub remark: String,
    pub cover_image: String,
    pub seq: isize,
    pub duration: u32, 
    pub state: u32,
    pub created: u32, 
    //pub updated: u32,
    pub category_id: usize,
    pub tag_ids: String,
    pub author_id: usize,
    pub url: String,
    pub is_recommended: u32,
}

impl Model for Videos { 
    fn get_table_name() -> &'static str { "videos" }
}

impl ModelBackend for Videos { 

    type M = Self;

    get_fields!(Self, [
        title => String,
        remark => String,
        cover_image => String,
        url => String,
        tag_ids => String,
        seq => isize,
        duration => u32,
        state => u32,
        //created => u32,
        //updated => u32,
        category_id => usize,
        author_id => usize,
        is_recommended => u32,
    ]);

    fn validate(data: &HashMap<String, String>) -> Result<(), String> { 
        Validator::load(&data)
            .string_length("title", "The title length must be between 2-30", 2, 50, true)
            .string_limit("remark", "Remarks cannot exceed 200", 200)
            .string_length("cover_image", "The cover address length must be between 2-200", 2, 200, true)
            .is_unsigned("duration", "Time must be the correct number")
            .is_numeric("seq", "Sort must be effective numbers")
            .is_state("state", "You must choose the correct state value")
            .validate()
    }
}
