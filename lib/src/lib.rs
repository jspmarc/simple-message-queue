pub mod enums;
pub mod structs;
pub mod traits;

#[cfg(test)]
mod tests {
    use std::any::{Any};
    use std::rc::Rc;
    use bytes::Bytes;
    use crate::enums::code::Code;
    use crate::enums::r#type::Type;
    use crate::structs::message::{Message, Metadata};

    #[test]
    fn message_serialize_success() {
        let msg = Message {
            metadata: Metadata {
                r#type: Type::U16(0),
                code: Code::SUCCESS,
                size: 2,
            },
            data: Rc::new(vec![1, 127]),
        };

        let expected = Bytes::from(vec![0b0000_0001,
                                        0, 0, 0, 2,
                                        1, 127]);

        assert_eq!(expected, msg.serialize());
    }

    #[test]
    fn message_deserialize_success() {
        let msg = Bytes::from(vec![0b0000_0001,
                                   0, 0, 0, 2,
                                   1, 127]);

        let expected = Message {
            metadata: Metadata {
                r#type: Type::U16(0),
                code: Code::SUCCESS,
                size: 2,
            },
            data: Rc::new(vec![1, 127]),
        };

        assert_eq!(Message::deserialize(&msg), expected);
    }
}
