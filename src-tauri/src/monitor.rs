use crate::border::Border;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Monitor {
    pub name: String,
    pub borders: Border,
    pub width: i32,
    pub height: i32,
}