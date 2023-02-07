use std::any::Any;
use bytes::Bytes;
use num::{Float, Integer};
use crate::enums::r#type::Type;
use crate::enums::code::Code;
use crate::structs::message::{Message, Metadata};

impl Eq for Message {}

impl Eq for Metadata {}

pub(in super::super) fn map_bits_to_type(bits: u8) -> Type {
    if bits == 0b0000 {
        Type::u8
    } else if bits == 0b0001 {
        Type::u16
    } else if bits == 0b0010 {
        Type::u32
    } else if bits == 0b0011 {
        Type::u64
    } else if bits == 0b0100 {
        Type::i8
    } else if bits == 0b0101 {
        Type::i16
    } else if bits == 0b0110 {
        Type::i32
    } else if bits == 0b0111 {
        Type::i64
    } else if bits == 0b1000 {
        Type::f32
    } else if bits == 0b1001 {
        Type::f64
    } else if bits == 0b1010 {
        Type::str
    } else {
        unimplemented!()
    }
}

pub(in super::super) fn map_type_to_bits(ty: &Type) -> u8 {
    (match ty {
        Type::u8 => 0b0000,
        Type::u16 => 0b0001,
        Type::u32 => 0b0010,
        Type::u64 => 0b0011,
        Type::i8 => 0b0100,
        Type::i16 => 0b0101,
        Type::i32 => 0b0110,
        Type::i64 => 0b0111,
        Type::f32 => 0b1000,
        Type::f64 => 0b1001,
        Type::str => 0b1010,
    }) & 0x0F
}

pub(in super::super) fn map_bits_to_code(bits: u8) -> Code {
    if bits == 0b0000 << 4 {
        Code::SUCCESS
    } else if bits == 0b0001 << 4 {
        Code::NULL_DATA
    } else if bits == 0b0010 {
        Code::EMPTY_QUEUE
    } else {
        unimplemented!()
    }
}

pub(in super::super) fn map_code_to_bits(co: &Code) -> u8 {
    (match co {
        Code::SUCCESS => 0b0000 << 4,
        Code::NULL_DATA => 0b0001 << 4,
        Code::EMPTY_QUEUE => 0b0010 << 4,
    }) & 0xF0
}