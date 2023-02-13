use crate::enums::r#type::Type;
use crate::enums::code::Code;
use crate::enums::errors::MessageError;
use crate::structs::message::{Message, Metadata};

impl Eq for Message {}

impl Eq for Metadata {}

#[inline]
/// assume the given nibble are in-spec
pub(in super::super) fn map_nibble_to_type(nibble: u8) -> Type {
    if nibble == 0b0000 {
        Type::U8(0)
    } else if nibble == 0b0001 {
        Type::U16(0)
    } else if nibble == 0b0010 {
        Type::U32(0)
    } else if nibble == 0b0011 {
        Type::U64(0)
    } else if nibble == 0b0100 {
        Type::I8(0)
    } else if nibble == 0b0101 {
        Type::I16(0)
    } else if nibble == 0b0110 {
        Type::I32(0)
    } else if nibble == 0b0111 {
        Type::I64(0)
    } else if nibble == 0b1000 {
        Type::F32(0.0)
    } else if nibble == 0b1001 {
        Type::F64(0.0)
    } else {
        // nibble == 0b1010
        Type::Str("".to_string())
    }
}

#[inline]
pub(in super::super) fn map_type_to_nibble(ty: &Type) -> u8 {
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

#[inline]
/// assume the given nibble are in-spec
pub(in super::super) fn map_nibble_to_code(nibble: u8) -> Code {
    if nibble == 0b0000 << 4 {
        Code::SUCCESS
    } else {
        // nibble == 0b0010
        Code::EMPTY_QUEUE
    }
}

#[inline]
pub(in super::super) fn map_code_to_nibble(co: &Code) -> u8 {
    (match co {
        Code::SUCCESS => 0b0000 << 4,
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

macro_rules! bytes_to_num {
    ($arr:expr, $Type:expr, $ty:ty, 1) => {
        $Type(<$ty>::from_be_bytes([$arr[0]]))
    };

    ($arr:expr, $Type:expr, $ty:ty, 2) => {
        $Type(<$ty>::from_be_bytes([$arr[0], $arr[1]]))
    };

    ($arr:expr, $Type:expr, $ty:ty, 4) => {
        $Type(<$ty>::from_be_bytes([$arr[0], $arr[1], $arr[2], $arr[3]]))
    };

    ($arr:expr, $Type:expr, $ty:ty, 8) => {
        $Type(<$ty>::from_be_bytes([$arr[0], $arr[1], $arr[2], $arr[3],
                                  $arr[4], $arr[5], $arr[6], $arr[7]]))
    };
}

pub(in super::super) fn parse_num(data: &Vec<u8>, ty: Type) -> Vec<Type> {
    let size: usize = ty.get_size();
    let mut ret_val: Vec<Type> = vec![];

    let mut i = 0;
    while i < data.len() {
        let current_bytes = &data[i..i + size];
        let current_data = match ty {
            Type::U8(_) => bytes_to_num!(current_bytes, Type::U8, u8, 1),
            Type::U16(_) => bytes_to_num!(current_bytes, Type::U16, u16, 2),
            Type::U32(_) => bytes_to_num!(current_bytes, Type::U32, u32, 4),
            Type::U64(_) => bytes_to_num!(current_bytes, Type::U64, u64, 8),
            Type::I8(_) => bytes_to_num!(current_bytes, Type::I8, i8, 1),
            Type::I16(_) => bytes_to_num!(current_bytes, Type::I16, i16, 2),
            Type::I32(_) => bytes_to_num!(current_bytes, Type::I32, i32, 4),
            Type::I64(_) => bytes_to_num!(current_bytes, Type::I64, i64, 8),
            Type::F32(_) => bytes_to_num!(current_bytes, Type::F32, f32, 4),
            Type::F64(_) => bytes_to_num!(current_bytes, Type::F64, f64, 8),
            _ => unimplemented!()
        };

        ret_val.push(current_data);

        i += size;
    }

    ret_val
}

pub(in super::super) fn validate_header(header: &[u8]) -> Result<(), MessageError> {
    let first_byte = header[0];

    let first_nibble = first_byte >> 4;
    if first_nibble != 0b0000 && first_nibble != 0b0010 {
        return Err(MessageError::InvalidHeaderBits);
    }

    let second_nibble = first_byte & 0x0F;
    if second_nibble > 0b1010 {
        return Err(MessageError::InvalidHeaderBits);
    }

    Ok(())
}

pub(in super::super) fn validate_body(body: &[u8], len: usize, ty: &Type) -> Result<(), MessageError> {
    let size = ty.get_size();

    if size == 0 {
        return if body[body.len() - 1] == 0 {
            Ok(())
        } else {
            Err(MessageError::InvalidData)
        }
    }

    let expected_size = len * size;
    if expected_size != body.len() {
        return Err(MessageError::InvalidDataLength)
    }

    Ok(())
}

macro_rules! downcast_type {
    ($data:ident, $ty:ty) => {
        Ok($data.iter()
            .map(|x| *(x.get_value().downcast::<$ty>().unwrap()))
            .collect::<Vec<$ty>>())
    };
}

macro_rules! generate_constructor_from_number {
    ($($ty:ty, $name:ident, $r#type:expr),+) => {
        $(pub fn $name(data: &[$ty]) -> Self {
            let mut msg_data = vec![];
            data.iter().for_each(|x| msg_data.extend_from_slice(&x.to_be_bytes()));

            Message {
                metadata: Metadata {
                    r#type: $r#type,
                    code: Code::SUCCESS,
                    size: data.len(),
                },
                data: Rc::new(msg_data),
            }
        })+
    };
}

macro_rules! generate_parser_to_number {
    ($($ty:ty, $name:ident, $type_pat:pat, $type_expr:expr),+) => {
        $(pub fn $name(&self) -> Result<Vec<$ty>, MessageError> {
            if let $type_pat = self.metadata.r#type {
                let data = parse_num(&*self.data, $type_expr);
                return downcast_type!(data, $ty);
            }

            Err(MessageError::InvalidType)
        })+
    };
}

pub(in super::super) use {
    downcast_type,
    generate_constructor_from_number,
    generate_parser_to_number,
};
