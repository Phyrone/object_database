use std::future::Future;

pub trait Serializable {
     fn serialize(&self, writer: &dyn ObjectWriter) ;
     fn deserialize(reader: &mut dyn ObjectReader) -> Self;
}

pub trait ObjectWriter {}
pub trait ObjectReader {}
