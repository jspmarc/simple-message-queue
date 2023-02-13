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

    #[inline]
    /// for strings and other type with variable size, we send size 0
    pub fn get_size(&self) -> usize {
        match self {
            Type::U8(_) | Type::I8(_) => 1,
            Type::U16(_) | Type::I16(_) => 2,
            Type::U32(_) | Type::I32(_) | Type::F32(_) => 4,
            Type::U64(_) | Type::I64(_) | Type::F64(_) => 8,
            Type::Str(_) => 0,
        }
    }
}