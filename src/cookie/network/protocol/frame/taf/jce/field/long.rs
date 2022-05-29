use bytes::{Buf, BufMut, Bytes, BytesMut};

use super::{BYTE, HeadData, INT, JceType, JLong, LONG, SHORT, TYPE_ERR, ZERO_TAG};

impl JceType<JLong> for JLong {
    fn to_bytes(&self, tag: u8) -> BytesMut {
        if *self < 2147483648 && *self >= -2147483648 { return (*self as i32).to_bytes(tag); }
        let mut b = HeadData::new(LONG, tag, 8).format();
        b.put_i64(*self);
        b
    }

    fn from_bytes(b: &mut Bytes, r#type: u8) -> JLong {
        match r#type {
            BYTE => b.get_i8() as i64,
            SHORT => b.get_i16() as i64,
            INT => b.get_i32() as i64,
            LONG => b.get_i64(),
            ZERO_TAG => 0,
            _ => panic!("{}", TYPE_ERR),
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use super::{INT, JceType, JLong, LONG};

    #[test]
    fn to_bytes() {
        assert_eq!(1145141919810_i64.to_bytes(0), vec![3, 0, 0, 1, 10, 159, 199, 0, 66]);
    }

    #[test]
    fn from_bytes() {
        assert_eq!(
            JLong::from_bytes(&mut Bytes::from(vec![0, 0, 1, 10, 159, 199, 0, 66]), LONG),
            1145141919810_i64,
        );
    }

    #[test]
    fn to_bytes_int() { assert_eq!(114514_i64.to_bytes(0), vec![2, 0, 1, 191, 82]); }

    #[test]
    fn from_bytes_int() {
        assert_eq!(
            JLong::from_bytes(&mut Bytes::from(vec![0, 1, 191, 82]), INT),
            114514_i64,
        );
    }
}