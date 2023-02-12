use std::any::Any;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Str(String),
}

impl Type {
    pub fn get_value(&self) -> Box<dyn Any> {
        match self {
            Type::U8(u8) => Box::new(*u8),
            Type::U16(u16) => Box::new(*u16),
            Type::U32(u32) => Box::new(*u32),
            Type::U64(u64) => Box::new(*u64),
            Type::I8(i8) => Box::new(*i8),
            Type::I16(i16) => Box::new(*i16),
            Type::I32(i32) => Box::new(*i32),
            Type::I64(i64) => Box::new(*i64),
            Type::F32(f32) => Box::new(*f32),
            Type::F64(f64) => Box::new(*f64),
            Type::Str(str) => Box::new(str.to_owned()),
        }
    }
}