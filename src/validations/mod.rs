use std::collections::HashMap;
use regex::Regex;
use std::str::FromStr;

lazy_static! { 
    /// User name regular
    static ref RE_USERNAME: Regex = Regex::new(r"^[a-zA-Z]+[a-zA-Z_0-9]{4,19}$").unwrap();
    /// Email regular
    static ref RE_MAIL: Regex = Regex::new(r"/^([A-Za-z0-9_\-\.])+\@([A-Za-z0-9_\-\.])+\.([A-Za-z]{2,5})$/").unwrap();
}

#[derive(Debug)]
pub struct Validator<'a> { 
    errors: Vec<&'static str>,
    data: &'a HashMap<String, String>
}

pub trait Validation { 
    fn validate(_data: &HashMap<String, String>) -> Result<(), String> { 
        Ok(())
    }
}

impl<'a> Validator<'a> { 
    
    pub fn load(data: &'a HashMap<String, String>) -> Self { 
        Self { 
            errors: vec![],
            data: data,
        }
    }

    /// Whether it is the user name, 6-20 bits, the beginning of English, the number, the bottom line, the English
    pub fn is_username(&mut self, field: &'static str, message: &'static str, is_required: bool) -> &mut Self { 
        if let Some(v) = self.data.get(field) { 
            let count = v.chars().count();
            if count >= 5 && count < 20 && RE_USERNAME.is_match(v) { 
                return self;
            } 
        }
        if is_required { 
            self.errors.push(message);
        }
        self
    }

    /// Detect whether it is the correct password format
    pub fn is_password(&mut self, field: &'static str, message: &'static str) -> &mut Self { 
        if let Some(v) = self.data.get(field) { 
            let count = v.chars().count();
            if count > 5 && count < 20 { 
                return self;
            }
        }
        self.errors.push(message);
        self
    }

    /// Detect whether it is the correct verification code
    #[allow(dead_code)]
    pub fn is_check_code(&mut self, field: &'static str, message: &'static str) -> &mut Self { 
        if let Some(v) = self.data.get(field) { 
            if let Err(_) = v.parse::<usize>() { 
                self.errors.push(message);
            }
        } else { 
            self.errors.push(message);
        }
        self
    }

    /// Determine whether it is a number
    pub fn is_numeric(&mut self, field: &'static str, message: &'static str) -> &mut Self { 
        if let Some(v) = self.data.get(field) { 
            if let Ok(_) = v.parse::<isize>() { 
                return self;
            }
        } 
        self.errors.push(message);
        self
    }

    /// The judgment must be positive integer (including 0)
    pub fn is_unsigned(&mut self, field: &'static str, message: &'static str) -> &mut Self { 
        if let Some(v) = self.data.get(field) { 
            if let Ok(_) = v.parse::<usize>() { 
                return self;
            }
        }
        self.errors.push(message);
        self
    }

    /// The content of the two inputs must be consistent
    pub fn equal(&mut self, field: &'static str, equal_field: &'static str, message: &'static str) -> &mut Self { 
        if let Some(v) = self.data.get(field) { 
            if let Some(e) = self.data.get(equal_field) { 
                if v == e { 
                    return self;
                }
            }
        }
        self.errors.push(message);
        self
    }

    /// Whether it is 1/0 option
    pub fn is_yes_no(&mut self, field: &'static str, message: &'static str) -> &mut Self { 
        if let Some(v) = self.data.get(field) { 
            if let Ok(n) = v.parse::<usize>() { 
                if n == 0 || n == 1 { 
                    return self;
                }
            }
        }
        self.errors.push(message);
        self
    }

    /// Whether it is 1/0 option
    pub fn is_state(&mut self, field: &'static str, message: &'static str) -> &mut Self { 
        if let Some(v) = self.data.get(field) { 
            if let Ok(n) = v.parse::<usize>() { 
                if n == 0 || n == 1 { 
                    return self;
                }
            }
        }
        self.errors.push(message);
        self
    }

    /// It must be within a certain range
    pub fn in_range<T: Sized + FromStr + PartialEq>(&mut self, field: &'static str, message: &'static str, array: &[T]) -> &mut Self { 
        if let Some(v) = self.data.get(field) { 
            if let Ok(n) = v.parse::<T>() { 
                if array.contains(&n) { 
                    return self;
                }
            }
        }
        self.errors.push(message);
        self
    }

    // Determine whether it is an email
    //pub fn is_mail(&mut self, field: &'static str, message: &'static str, is_required: bool) -> &mut Self  {
    //    if let Some(v) = self.data.get(field) { 
    //        if RE_MAIL.is_match(v) { 
    //            self.errors.push(message);
    //        }
    //    } else if is_required { 
    //        self.errors.push(message);
    //    }
    //    self
    //}

    /// Specify string of length
    pub fn string_length(&mut self, field: &'static str, message: &'static str, min: usize, max: usize, is_required: bool) -> &mut Self { 
        if let Some(v) = self.data.get(field) { 
            let word_count = v.chars().count();
            if word_count < min || word_count > max {
                self.errors.push(message);
            }
        } else if is_required { 
            self.errors.push(message);
        }
        self
    }

    /// Limited length string
    pub fn string_limit(&mut self, field: &'static str, message: &'static str, max: usize) -> &mut Self { 
        if let Some(v) = self.data.get(field) { 
            if v.len() > max { 
                self.errors.push(message);
            }
        }
        self
    }

    /// Execute
    pub fn validate(&mut self) -> Result<(), String> { 
        if self.errors.len() > 0 { 
            return Err(self.errors.join(","));
        }
        Ok(())
    }
}
