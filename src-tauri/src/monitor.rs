use crate::border::Border;
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug)]
pub(crate) struct Monitor {
    pub name: String,
    pub borders: Border,
    pub width: i32,
    pub height: i32 
}

impl serde::Serialize for Monitor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Monitor", 2)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("borders", &self.borders)?;
        s.serialize_field("width", &self.width)?;
        s.serialize_field("height", &self.height)?;
        s.end()
    }
}
