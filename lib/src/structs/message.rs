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

macro_rules! downcast_type {
    ($data:ident, $ty:ty) => {
        $data.iter()
            .map(|x| *(x.get_value().downcast::<$ty>().unwrap()))
            .collect::<Vec<$ty>>()
    };
}

macro_rules! parse_precheck {
    ($metadata:expr) => {
        if $metadata.code == Code::NULL_DATA {
            return None;
        }

        if $metadata.size == 0 {
            return Some(vec![]);
        }
    };
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

impl Message {
    pub fn parse_data_to_str(&self) -> Vec<String> {
        if let Type::Str(_) = self.metadata.r#type {
            let data = parse_string(&*self.data);
            return downcast_type!(data, String);
        }

        unimplemented!()
    }

    pub fn parse_data_to_u8(&self) -> Vec<u8> {
        if let Type::U8(_) = self.metadata.r#type {
            let data = parse_num(&*self.data, Type::U8(0));
            return downcast_type!(data, u8);
        }

        unimplemented!()
    }

    pub fn parse_data_to_u16(&self) -> Vec<u16> {
        if let Type::U16(_) = self.metadata.r#type {
            let data = parse_num(&*self.data, Type::U16(0));
            return downcast_type!(data, u16);
        }

        unimplemented!()
    }

    pub fn parse_data_to_u32(&self) -> Vec<u32> {
        if let Type::U32(_) = self.metadata.r#type {
            let data = parse_num(&*self.data, Type::U32(0));
            return downcast_type!(data, u32);
        }

        unimplemented!()
    }

    pub fn parse_data_to_u64(&self) -> Vec<u64> {
        if let Type::U64(_) = self.metadata.r#type {
            let data = parse_num(&*self.data, Type::U64(0));
            return downcast_type!(data, u64);
        }

        unimplemented!()
    }

    pub fn parse_data_to_i8(&self) -> Vec<i8> {
        if let Type::I8(_) = self.metadata.r#type {
            let data = parse_num(&*self.data, Type::I8(0));
            return downcast_type!(data, i8);
        }

        unimplemented!()
    }

    pub fn parse_data_to_i16(&self) -> Vec<i16> {
        if let Type::I16(_) = self.metadata.r#type {
            let data = parse_num(&*self.data, Type::I16(0));
            return downcast_type!(data, i16);
        }

        unimplemented!()
    }

    pub fn parse_data_to_i32(&self) -> Vec<i32> {
        if let Type::I32(_) = self.metadata.r#type {
            let data = parse_num(&*self.data, Type::I32(0));
            return downcast_type!(data, i32);
        }

        unimplemented!()
    }

    pub fn parse_data_to_i64(&self) -> Vec<i64> {
        if let Type::I64(_) = self.metadata.r#type {
            let data = parse_num(&*self.data, Type::I64(0));
            return downcast_type!(data, i64);
        }

        unimplemented!()
    }

    pub fn parse_data_to_f32(&self) -> Vec<f32> {
        if let Type::F32(_) = self.metadata.r#type {
            let data = parse_num(&*self.data, Type::F32(0.0));
            return downcast_type!(data, f32);
        }

        unimplemented!()
    }

    pub fn parse_data_to_f64(&self) -> Vec<f64> {
        if let Type::F64(_) = self.metadata.r#type {
            let data = parse_num(&*self.data, Type::F64(0.0));
            return downcast_type!(data, f64);
        }

        unimplemented!()
    }
}
