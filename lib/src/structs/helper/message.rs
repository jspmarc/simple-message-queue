use crate::enums::r#type::Type;
use crate::enums::code::Code;
use crate::structs::message::{Message, Metadata};

impl Eq for Message {}

impl Eq for Metadata {}

pub(in super::super) fn map_bits_to_type(bits: u8) -> Type {
    if bits == 0b0000 {
        Type::U8(0)
    } else if bits == 0b0001 {
        Type::U16(0)
    } else if bits == 0b0010 {
        Type::U32(0)
    } else if bits == 0b0011 {
        Type::U64(0)
    } else if bits == 0b0100 {
        Type::I8(0)
    } else if bits == 0b0101 {
        Type::I16(0)
    } else if bits == 0b0110 {
        Type::I32(0)
    } else if bits == 0b0111 {
        Type::I64(0)
    } else if bits == 0b1000 {
        Type::F32(0.0)
    } else if bits == 0b1001 {
        Type::F64(0.0)
    } else if bits == 0b1010 {
        Type::Str("".to_string())
    } else {
        unimplemented!()
    }
}

pub(in super::super) fn map_type_to_bits(ty: &Type) -> u8 {
    (match ty {
        Type::U8(_) => 0b0000,
        Type::U16(_) => 0b0001,
        Type::U32(_) => 0b0010,
        Type::U64(_) => 0b0011,
        Type::I8(_) => 0b0100,
        Type::I16(_) => 0b0101,
        Type::I32(_) => 0b0110,
        Type::I64(_) => 0b0111,
        Type::F32(_) => 0b1000,
        Type::F64(_) => 0b1001,
        Type::Str(_) => 0b1010,
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

pub(in super::super) fn parse_string(data: &[u8]) -> Vec<Type> {
    let mut left_ptr: usize = 0;
    let mut right_ptr: usize;
    let mut ret_val: Vec<Type> = vec![];

    for (i, datum) in data.iter().enumerate() {
        if *datum == 0 {
            right_ptr = i - 1;

            let data = &data[left_ptr..right_ptr + 1];
            let data = String::from_utf8_lossy(data).to_string();
            ret_val.push(Type::Str(data.to_owned()));

            left_ptr = i + 1;
        }
    }

    ret_val
}