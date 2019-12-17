use crate::db::Connection;


pub struct Context {
    pub connection: Connection,
}

impl juniper::Context for Context {}

impl AsRef<Self> for Context {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}
