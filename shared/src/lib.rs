use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    an_int: i32,
    a_string: String,
    a_bool: bool,
}

impl Message {
    pub fn new(an_int: i32, a_string: String, a_bool: bool) -> Self {
        Self {
            an_int,
            a_string,
            a_bool,
        }
    }

    pub fn increment(&self) -> Self {
        Self {
            an_int: self.an_int + 1,
            a_string: self.a_string.clone() + "!",
            a_bool: !self.a_bool,
        }
    }
}
