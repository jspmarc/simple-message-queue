use std::rc::Rc;
use bytes::Bytes;
use crate::enums::r#type::Type;
use crate::enums::code::Code;
use crate::structs::helper::message::*;

#[derive(Debug, PartialEq)]
pub struct Message {
    pub(crate) metadata: Metadata,
    pub(crate) data: Rc<Vec<u8>>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Metadata {
    pub(crate) r#type: Type,
    pub(crate) code: Code,
    pub(crate) size: usize,
}

impl Message {
    pub fn get_type(&self) -> &Type {
        &self.metadata.r#type
    }

    pub fn get_code(&self) -> Code {
        self.metadata.code
    }

    pub fn get_size(&self) -> usize {
        self.metadata.size
    }

    pub fn get_data(&self) -> Rc<Vec<u8>> {
        Rc::clone(&self.data)
    }

    pub fn get_parsed_data(&self) -> Option<Vec<Type>> {
        let metadata = &self.metadata;
        if metadata.size == 0 || metadata.code == Code::NULL_DATA {
            return None;
        }

        let data = &*self.data;

        match metadata.r#type {
            Type::Str(_) => Some(parse_string(data)),
            // Type::u8 => Some(parse_int(data, false, 8)),
            // Type::u16 => Some(parse_int(data, false, 16)),
            // Type::u32 => Some(parse_int(data, false, 32)),
            // Type::u64 => Some(parse_int(data, false, 64)),
            // Type::i8 => Some(parse_int(data, true, 8)),
            // Type::i16 => Some(parse_int(data, true, 16)),
            // Type::i32 => Some(parse_int(data, true, 32)),
            // Type::i64 => Some(parse_int(data, true, 64)),
            _ => Some(vec![Type::U8(0)]),
        }
    }

    pub fn serialize(&self) -> Bytes {
        let metadata = {
            let metadata = &self.metadata;
            let code = map_code_to_bits(&metadata.code);
            let ty = map_type_to_bits(&metadata.r#type);
            let first_byte = code + ty;

            let size = metadata.size;
            let size_first_byte: u8 = (size & 0xFF00_0000) as u8;
            let size_second_byte: u8 = (size & 0x00FF_0000) as u8;
            let size_third_byte: u8 = (size & 0x0000_FF00) as u8;
            let size_fourth_byte: u8 = (size & 0x0000_00FF) as u8;

            Bytes::from(vec![first_byte,
                             size_first_byte,
                             size_second_byte,
                             size_third_byte,
                             size_fourth_byte])
        };

        Bytes::from([metadata, Bytes::from((*self.data).clone())].concat())
    }

    pub fn deserialize(message: &Bytes) -> Message {
        let first_byte = message[0];
        let code = map_bits_to_code(first_byte & 0xF0);
        let r#type = map_bits_to_type(first_byte & 0x0F);
        let size_bytes = message.slice(1..5).to_vec();
        let size: usize = ((size_bytes[0] as usize) << 24) +
            ((size_bytes[1] as usize) << 16) +
            ((size_bytes[2] as usize) << 8) +
            (size_bytes[3] as usize);
        let data = Rc::new(message.slice(5..).to_vec());

        Message {
            metadata: Metadata {
                r#type,
                code,
                size,
            },
            data,
        }
    }
}