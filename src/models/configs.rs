use fluffy::{DbRow, model::Model,};
use super::ModelBackend;
use serde_derive::{Serialize};
use std::collections::HashMap;
use crate::validations::Validator;

#[derive(Default, Debug, Serialize)]
pub struct Configs { 
    pub id: usize,
    pub site_name: String,
    pub site_url: String, 
    pub seo_keyword: String,
    pub seo_desc: String,
    pub copyright: String,
}

impl Model for Configs { 
    fn get_table_name() -> &'static str { "configs" }
}

impl ModelBackend for Configs { 

    type M = Self;

    get_fields!(Self, [
        site_name => String,
        site_url => String,
        seo_keyword => String,
        seo_desc => String,
        copyright => String,
    ]);

    fn validate(data: &HashMap<String, String>) -> Result<(), String> { 
        Validator::load(&data)
            .string_length("site_name", "The classification name must be between 2-20", 2, 20, true)
            .string_length("site_url", "The link address cannot be empty", 1, 200, true)
            .string_limit("seo_keyword", "SEO explains that the length cannot exceed 200", 200)
            .string_limit("seo_desc", "SEO description length cannot exceed 200", 200)
            .string_limit("copyright", "The length of copyright information cannot exceed 200", 200)
            .validate()
    }
}
