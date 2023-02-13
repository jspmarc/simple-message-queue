pub mod enums;
pub mod structs;
pub mod traits;

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use bytes::Bytes;
    use crate::structs::message::{Message};

    #[test]
    fn message_serialize_success() {
        let msg = Message::from_u16_arr(&[1, 127]);
        let expected = Bytes::from(vec![0b0000_0001,
                                        0, 0, 0, 2,
                                        0, 1, 0, 127]);

        assert_eq!(expected, msg.serialize());
    }

    #[test]
    fn message_deserialize_success() {
        let msg = Bytes::from(vec![0b0000_0001,
                                   0, 0, 0, 2,
                                   0, 1, 0, 127]);

        let expected = Message::from_u16_arr(&[1, 127]);

        assert_eq!(Message::deserialize(&msg).unwrap(), expected);
    }

    #[test]
    fn message_parse_data_to_str_success_from_str_arr() {
        let expected = vec![
            String::from("Hello, World!"),
            String::from("Hi, World!"),
        ];

        let msg = Message::from_str_arr(&expected);
        let parsed = msg.parse_data_to_str();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn message_parse_data_to_str_success_from_str_single() {
        let s = "Halo";
        let msg = Message::from_str(s).unwrap();
        let parsed = msg.parse_data_to_str();

        assert_eq!(vec![s], parsed);
    }

    #[test]
    fn message_parse_data_to_u8_success() {
        let expected = vec![u8::MIN, u8::MAX];
        let msg = Message::from_u8_arr(&expected);

        let parsed = msg.parse_data_to_u8();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn message_parse_data_to_i8_success() {
        let expected = vec![i8::MIN, i8::MAX];
        let msg = Message::from_i8_arr(&expected);

        let parsed = msg.parse_data_to_i8();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn message_parse_data_to_u16_success() {
        let expected  = vec![u16::MIN, u16::MAX];
        let msg = Message::from_u16_arr(&expected);

        let parsed = msg.parse_data_to_u16();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn message_parse_data_to_i16_success() {
        let expected  = vec![i16::MIN, i16::MAX];
        let msg = Message::from_i16_arr(&expected);

        let parsed = msg.parse_data_to_i16();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn message_parse_data_to_u32_success() {
        let expected  = vec![u32::MIN, u32::MAX];
        let msg = Message::from_u32_arr(&expected);

        let parsed = msg.parse_data_to_u32();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn message_parse_data_to_i32_success() {
        let expected  = vec![i32::MIN, i32::MAX];
        let msg = Message::from_i32_arr(&expected);

        let parsed = msg.parse_data_to_i32();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn message_parse_data_to_u64_success() {
        let expected  = vec![u64::MIN, u64::MAX];
        let msg = Message::from_u64_arr(&expected);

        let parsed = msg.parse_data_to_u64();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn message_parse_data_to_i64_success() {
        let expected  = vec![i64::MIN, i64::MAX];
        let msg = Message::from_i64_arr(&expected);

        let parsed = msg.parse_data_to_i64();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn message_parse_data_to_f32_success() {
        let expected  = vec![f32::MIN, f32::MAX];
        let msg = Message::from_f32_arr(&expected);

        let parsed = msg.parse_data_to_f32();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn message_parse_data_to_f64_success() {
        let expected  = vec![f64::MIN, f64::MAX];
        let msg = Message::from_f64_arr(&expected);

        let parsed = msg.parse_data_to_f64();

        assert_eq!(expected, parsed);
    }
}
