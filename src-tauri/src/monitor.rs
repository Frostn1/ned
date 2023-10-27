use crate::border::Border;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct Monitor {
    pub name: String,
    pub borders: Border,
}

impl serde::Serialize for Monitor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Monitor", 2)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("borders", &self.borders)?;
        s.end()
    }
}
