pub mod enums;
pub mod structs;
pub mod traits;

#[cfg(test)]
mod tests {
    use crate::enums::errors::MessageError;
    use crate::structs::message::Message;
    use bytes::Bytes;
    use std::str::FromStr;

    #[test]
    fn message_serialize_success() {
        let msg = Message::from_u16_arr(&[1, 127]);
        let expected = Bytes::from(vec![0b0000_0001, 0, 0, 0, 2, 0, 1, 0, 127]);

        assert_eq!(expected, msg.serialize());
    }

    #[test]
    fn message_deserialize_success() {
        let msg = [0b0000_0001, 0, 0, 0, 2, 0, 1, 0, 127];

        let expected = Message::from_u16_arr(&[1, 127]);

        assert_eq!(Message::deserialize(&msg).unwrap(), expected);
    }

    #[test]
    fn message_deserialize_invalid_header() {
        let msg = [0b1000_0001, 0, 0, 0, 2, 0, 1, 0, 127];

        let res = Message::deserialize(&msg);
        assert_eq!(res.unwrap_err(), MessageError::InvalidHeaderBits);
    }

    #[test]
    fn message_deserialize_invalid_data_length() {
        let msg = [0b0000_0001, 0, 0, 0, 3, 0, 1, 0, 127];

        let res = Message::deserialize(&msg);
        assert_eq!(res.unwrap_err(), MessageError::InvalidDataLength);
    }

    #[test]
    fn message_deserialize_invalid_data() {
        let msg = [0b0000_1010, 0, 0, 0, 1, 1];

        let res = Message::deserialize(&msg);
        assert_eq!(res.unwrap_err(), MessageError::InvalidData);
    }

    #[test]
    fn message_parse_data_to_str_success_from_str_arr() {
        let expected = vec![String::from("Hello, World!"), String::from("Hi, World!")];

        let msg = Message::from_str_arr(&expected);
        let parsed = msg.parse_data_to_str().unwrap();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn message_parse_data_to_str_success_from_str_single() {
        let s = "Halo";
        let msg = Message::from_str(s).unwrap();
        let parsed = msg.parse_data_to_str().unwrap();

        assert_eq!(vec![s], parsed);
    }

    #[test]
    fn message_parse_data_to_str_err() {
        let expected = vec![u8::MIN, u8::MAX];
        let msg = Message::from_u8_arr(&expected);
        let parsed = msg.parse_data_to_str().unwrap_err();

        assert_eq!(parsed, MessageError::InvalidType);
    }

    #[test]
    fn message_parse_data_to_num_err() {
        let expected = vec![u8::MIN, u8::MAX];
        let msg = Message::from_u8_arr(&expected);
        let parsed = msg.parse_data_to_f32().unwrap_err();

        assert_eq!(parsed, MessageError::InvalidType);
    }

    #[test]
    fn message_parse_data_to_u8_success() {
        let expected = vec![u8::MIN, u8::MAX];
        let msg = Message::from_u8_arr(&expected);

        let parsed = msg.parse_data_to_u8().unwrap();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn message_parse_data_to_i8_success() {
        let expected = vec![i8::MIN, i8::MAX];
        let msg = Message::from_i8_arr(&expected);

        let parsed = msg.parse_data_to_i8().unwrap();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn message_parse_data_to_u16_success() {
        let expected = vec![u16::MIN, u16::MAX];
        let msg = Message::from_u16_arr(&expected);

        let parsed = msg.parse_data_to_u16().unwrap();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn message_parse_data_to_i16_success() {
        let expected = vec![i16::MIN, i16::MAX];
        let msg = Message::from_i16_arr(&expected);

        let parsed = msg.parse_data_to_i16().unwrap();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn message_parse_data_to_u32_success() {
        let expected = vec![u32::MIN, u32::MAX];
        let msg = Message::from_u32_arr(&expected);

        let parsed = msg.parse_data_to_u32().unwrap();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn message_parse_data_to_i32_success() {
        let expected = vec![i32::MIN, i32::MAX];
        let msg = Message::from_i32_arr(&expected);

        let parsed = msg.parse_data_to_i32().unwrap();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn message_parse_data_to_u64_success() {
        let expected = vec![u64::MIN, u64::MAX];
        let msg = Message::from_u64_arr(&expected);

        let parsed = msg.parse_data_to_u64().unwrap();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn message_parse_data_to_i64_success() {
        let expected = vec![i64::MIN, i64::MAX];
        let msg = Message::from_i64_arr(&expected);

        let parsed = msg.parse_data_to_i64().unwrap();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn message_parse_data_to_f32_success() {
        let expected = vec![f32::MIN, f32::MAX];
        let msg = Message::from_f32_arr(&expected);

        let parsed = msg.parse_data_to_f32().unwrap();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn message_parse_data_to_f64_success() {
        let expected = vec![f64::MIN, f64::MAX];
        let msg = Message::from_f64_arr(&expected);

        let parsed = msg.parse_data_to_f64().unwrap();

        assert_eq!(expected, parsed);
    }
}
