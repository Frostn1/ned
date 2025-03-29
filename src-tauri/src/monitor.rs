use crate::border::Border;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Monitor {
    pub name: String,
    pub border: Border,
    pub width: i32,
    pub height: i32,
}