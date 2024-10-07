use std::error::Error;

pub type VoidResult = Result<(), Box<dyn Error>>;
pub type F32Result = Result<f32, ()>;
pub type U32Result = Result<u32, ()>;
pub type U16Result = Result<u16, ()>;
pub type U8Result = Result<u8, ()>;
pub type VecU8Result = Result<Vec<u8>, ()>;
