use serde::ser::{Serialize, SerializeStruct, Serializer};


pub(crate) struct Border {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl serde::Serialize for Border {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Border", 4)?;
        s.serialize_field("left", &self.left)?;
        s.serialize_field("top", &self.top)?;
        s.serialize_field("right", &self.right)?;
        s.serialize_field("bottom", &self.bottom)?;
        s.end()
    }
}
