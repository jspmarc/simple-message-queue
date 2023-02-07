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

    #[test]
    fn message_get_parsed_data_list_str_success() {
        let expected = vec![
            String::from("Hello, World!"),
            String::from("Hi, World!"),
        ];
        let data = [
            Bytes::from(expected[0].clone()),
            Bytes::from("\0"),
            Bytes::from(expected[1].clone()),
            Bytes::from("\0"),
        ].concat();
        let msg = Message {
            metadata: Metadata {
                r#type: Type::Str(String::new()),
                code: Code::SUCCESS,
                size: expected.len(),
            },
            data: Rc::new(data),
        };

        let parsed = msg.get_parsed_data().unwrap();
        let result = parsed.iter()
            .map(|datum| {
                if let Type::Str(s) = datum {
                    s.clone()
                } else {
                    unimplemented!()
                }
            })
            .collect::<Vec<String>>();

        assert_eq!(expected.type_id(), result.type_id());
        assert_eq!(expected, result);
    }
}
