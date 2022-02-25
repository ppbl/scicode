use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub userid: i32,
    pub username: String,
    pub exp: usize,
}

pub static SECRET: &str = "balabala";
