use std::collections::HashMap;

/// page
pub const PAGE_IDS: [usize; 2] = [0, 1];

/// Location
pub const POSITION_IDS: [usize; 4] = [0, 1, 2, 3];

lazy_static! {
    pub static ref PAGES: HashMap<usize, &'static str> = {
        let mut data = HashMap::new();
        data.insert(0, "Home page");
        data.insert(1, "Detail page");
        data
    };
}

lazy_static! { 
    pub static ref POSITIONS: HashMap<usize, &'static str> = { 
        let mut data = HashMap::new();
        data.insert(0, "Top");
        data.insert(1, "Middle left");
        data.insert(2, "Middle right");
        data.insert(3, "Bottom");
        data
    };
}
