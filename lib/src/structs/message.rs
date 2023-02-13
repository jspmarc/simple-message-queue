use std::rc::Rc;
use std::str::FromStr;
use bytes::Bytes;
use crate::enums::r#type::Type;
use crate::enums::code::Code;
use crate::enums::errors::MessageError;
use crate::structs::helper::message::*;

#[derive(Debug, PartialEq)]
pub struct Message {
    metadata: Metadata,
    data: Rc<Vec<u8>>,
}

#[derive(Debug, PartialEq)]
pub(in super) struct Metadata {
    r#type: Type,
    code: Code,
    size: usize,
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
            let code = map_code_to_nibble(&metadata.code);
            let ty = map_type_to_nibble(&metadata.r#type);
            let first_byte = code + ty;

            let size = metadata.size;
            let size_first_byte = (size & 0xFF00_0000) as u8;
            let size_second_byte = (size & 0x00FF_0000) as u8;
            let size_third_byte = (size & 0x0000_FF00) as u8;
            let size_fourth_byte = (size & 0x0000_00FF) as u8;

            Bytes::from(vec![first_byte,
                             size_first_byte,
                             size_second_byte,
                             size_third_byte,
                             size_fourth_byte])
        };

        Bytes::from([metadata, Bytes::from((*self.data).clone())].concat())
    }

    pub fn deserialize(message: &Bytes) -> Result<Message, MessageError> {
        // header
        validate_header(&message[..5])?;
        let first_byte = message[0];
        let code = map_nibble_to_code(first_byte & 0xF0);
        let r#type = map_nibble_to_type(first_byte & 0x0F);
        let size_bytes = message.slice(1..5).to_vec();
        let size: usize = ((size_bytes[0] as usize) << 24) +
            ((size_bytes[1] as usize) << 16) +
            ((size_bytes[2] as usize) << 8) +
            (size_bytes[3] as usize);

        // body
        validate_body(&message[5..], size, &r#type)?;
        let data = message.slice(5..).to_vec();
        let data = Rc::new(data);

        Ok(Message {
            metadata: Metadata {
                r#type,
                code,
                size,
            },
            data,
        })
    }
}

impl FromStr for Message {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = [
            Bytes::from(s.to_string()),
            Bytes::from("\0".to_string())
        ].concat();

        Ok(Message {
            metadata: Metadata {
                r#type: Type::Str("".to_string()),
                code: Code::SUCCESS,
                size: 1,
            },
            data: Rc::new(data)
        })
    }
}

// constructors
impl Message {
    pub fn from_str_arr(data: &[String]) -> Self {
        let mut msg_data = vec![];

        for datum in data {
            msg_data.push(Bytes::from(datum.to_owned()));
            msg_data.push(Bytes::from("\0".to_string()));
        }

        Message {
            metadata: Metadata {
                r#type: Type::Str("".to_string()),
                code: Code::SUCCESS,
                size: data.len(),
            },
            data: Rc::new(msg_data.concat()),
        }
    }

    generate_constructor_from_number!(
        u8, from_u8_arr, Type::U8(0),
        u16, from_u16_arr, Type::U16(0),
        u32, from_u32_arr, Type::U32(0),
        u64, from_u64_arr, Type::U64(0),
        i8, from_i8_arr, Type::I8(0),
        i16, from_i16_arr, Type::I16(0),
        i32, from_i32_arr, Type::I32(0),
        i64, from_i64_arr, Type::I64(0),
        f32, from_f32_arr, Type::F32(0.0),
        f64, from_f64_arr, Type::F64(0.0)
    );
}

// parsers
impl Message {
    pub fn parse_data_to_str(&self) -> Result<Vec<String>, MessageError> {
        if let Type::Str(_) = self.metadata.r#type {
            let data = parse_string(&*self.data);
            return downcast_type!(data, String);
        }

        Err(MessageError::InvalidType)
    }

    generate_parser_to_number!(
        u8, parse_data_to_u8, Type::U8(_), Type::U8(0),
        u16, parse_data_to_u16, Type::U16(_), Type::U16(0),
        u32, parse_data_to_u32, Type::U32(_), Type::U32(0),
        u64, parse_data_to_u64, Type::U64(_), Type::U64(0),
        i8, parse_data_to_i8, Type::I8(_), Type::I8(0),
        i16, parse_data_to_i16, Type::I16(_), Type::I16(0),
        i32, parse_data_to_i32, Type::I32(_), Type::I32(0),
        i64, parse_data_to_i64, Type::I64(_), Type::I64(0),
        f32, parse_data_to_f32, Type::F32(_), Type::F32(0.0),
        f64, parse_data_to_f64, Type::F64(_), Type::F64(0.0)
    );
}
