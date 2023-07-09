use std::collections::HashMap;
use crate::validations::{Validator};

pub struct Index {}

impl Index { 

    /// Detect user login
    pub fn check_login(data: &HashMap<String, String>) -> Result<(), String> { 
        Validator::load(data)
            .is_username("username", "The user name must be entered in the correct format", true)
            .is_password("password", "Must enter a password")
            .validate()
    }

    /// Detect and modify password
    pub fn check_change_pwd(data: &HashMap<String, String>) -> Result<(), String> { 
        Validator::load(data)
            .is_password("old_password", "You must enter the old password in the correct format")
            .is_password("password", "Must enter the correct format password")
            .is_password("re_password", "Must enter the duplicate password")
            .equal("password", "re_password", "The password you input must be consistent")
            .validate()
    }
}
