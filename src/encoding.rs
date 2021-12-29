use crate::Error;

pub const VARINT_MAX_LEN: usize = 10;

/// Decodes a varint from a slice, returning the remainder of the slice and the value.
#[inline]
pub fn read_varint(buf: &[u8]) -> Result<(&[u8], u64), Error> {
    if let Some(&byte) = buf.get(0) {
        if byte <= 0x7f {
            return Ok((&buf[1..], byte as u64));
        }
    }
    read_varint_1(buf)
}

#[allow(clippy::identity_op)]
fn read_varint_1(buf: &[u8]) -> Result<(&[u8], u64), Error> {
    if buf.len() >= VARINT_MAX_LEN {
        debug_assert!(buf[0] > 0x7f);
        let mut value = (buf[0] & 0x7f) as u64;
        value |= ((buf[1] & 0x7f) as u64) << (1 * 7);
        if buf[1] <= 0x7f {
            return Ok((&buf[2..], value));
        }
        value |= ((buf[2] & 0x7f) as u64) << (2 * 7);
        if buf[2] <= 0x7f {
            return Ok((&buf[3..], value));
        }
        value |= ((buf[3] & 0x7f) as u64) << (3 * 7);
        if buf[3] <= 0x7f {
            return Ok((&buf[4..], value));
        }
        value |= ((buf[4] & 0x7f) as u64) << (4 * 7);
        if buf[4] <= 0x7f {
            return Ok((&buf[5..], value));
        }
        value |= ((buf[5] & 0x7f) as u64) << (5 * 7);
        if buf[5] <= 0x7f {
            return Ok((&buf[6..], value));
        }
        value |= ((buf[6] & 0x7f) as u64) << (6 * 7);
        if buf[6] <= 0x7f {
            return Ok((&buf[7..], value));
        }
        value |= ((buf[7] & 0x7f) as u64) << (7 * 7);
        if buf[7] <= 0x7f {
            return Ok((&buf[8..], value));
        }
        value |= ((buf[8] & 0x7f) as u64) << (8 * 7);
        if buf[8] <= 0x7f {
            return Ok((&buf[9..], value));
        }
        value |= ((buf[9] & 0x01) as u64) << (9 * 7);
        if buf[9] <= 0x01 {
            return Ok((&buf[10..], value));
        }
        Err(Error)
    } else {
        read_varint_loop(buf)
    }
}

fn read_varint_loop(buf: &[u8]) -> Result<(&[u8], u64), Error> {
    let mut value = 0;
    let mut index = 0;
    while let Some(&byte) = buf.get(index) {
        value |= ((byte & 0x7f) as u64) << (index * 7);
        index += 1;
        if byte <= 0x7f {
            return Ok((&buf[index..], value));
        }
    }
    Err(Error)
}

pub mod zigzag {
    #[inline]
    pub fn decode_32(n: u32) -> i32 {
        (n >> 1) as i32 ^ -((n & 1) as i32)
    }

    #[inline]
    pub fn decode_64(n: u64) -> i64 {
        (n >> 1) as i64 ^ -((n & 1) as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::read_varint;
    use super::zigzag;
    use crate::Error;
    use core::ptr;

    #[test]
    fn read_varint_ok() {
        for (input, (expected_value, len)) in [
            (&b"\x00"[..], (0, 1)),
            (&b"\x01"[..], (1, 1)),
            (&b"\x7f"[..], (127, 1)),
            (&b"\xa2\x74"[..], (14882, 2)),
            (&b"\xbe\xf7\x92\x84\x0b"[..], (2961488830, 5)),
            (&b"\xbe\xf7\x92\x84\x1b"[..], (7256456126, 5)),
            (
                &b"\x80\xe6\xeb\x9c\xc3\xc9\xa4\x49"[..],
                (41256202580718336, 8),
            ),
            (
                &b"\x9b\xa8\xf9\xc2\xbb\xd6\x80\x85\xa6\x01"[..],
                (11964378330978735131, 10),
            ),
            (
                &b"\xff\xff\xff\xff\xff\xff\xff\xff\xff\x01"[..],
                (0xffffffffffffffff, 10),
            ),
        ] {
            let (buf, value) = read_varint(input).unwrap();
            assert!(ptr::eq(buf, &input[len..]));
            assert_eq!(value, expected_value);
        }
    }

    #[test]
    fn read_varint_invalid() {
        assert_eq!(read_varint(&b""[..]), Err(Error));
        assert_eq!(read_varint(&b"\xf0\xab"[..]), Err(Error));
        assert_eq!(read_varint(&b"\xf0\xab\xc9\x9a\xf8\xb2"[..]), Err(Error));
    }

    #[test]
    fn read_varint_overflow() {
        assert_eq!(
            read_varint(&b"\xff\xff\xff\xff\xff\xff\xff\xff\xff\x02"[..]),
            Err(Error)
        );
    }

    #[test]
    fn read_varint_too_many_bytes() {
        assert_eq!(
            read_varint(&b"\xff\xff\xff\xff\xff\xff\xff\xff\xff\x80\x00"[..]),
            Err(Error)
        );
    }

    #[test]
    fn zigzag_decode_32() {
        assert_eq!(zigzag::decode_32(0), 0);
        assert_eq!(zigzag::decode_32(1), -1);
        assert_eq!(zigzag::decode_32(2), 1);
        assert_eq!(zigzag::decode_32(3), -2);
        assert_eq!(zigzag::decode_32(0x7FFFFFFE), 0x3FFFFFFF_u32 as i32);
        assert_eq!(zigzag::decode_32(0x7FFFFFFF), 0xC0000000_u32 as i32);
        assert_eq!(zigzag::decode_32(0xFFFFFFFE), 0x7FFFFFFF_u32 as i32);
        assert_eq!(zigzag::decode_32(0xFFFFFFFF), 0x80000000_u32 as i32);
    }

    #[test]
    fn zigzag_decode_64() {
        assert_eq!(zigzag::decode_64(0), 0);
        assert_eq!(zigzag::decode_64(1), -1);
        assert_eq!(zigzag::decode_64(2), 1);
        assert_eq!(zigzag::decode_64(3), -2);
        assert_eq!(
            zigzag::decode_64(0x000000007FFFFFFE),
            0x000000003FFFFFFF_u64 as i64
        );
        assert_eq!(
            zigzag::decode_64(0x000000007FFFFFFF),
            0xFFFFFFFFC0000000_u64 as i64
        );
        assert_eq!(
            zigzag::decode_64(0x00000000FFFFFFFE),
            0x000000007FFFFFFF_u64 as i64
        );
        assert_eq!(
            zigzag::decode_64(0x00000000FFFFFFFF),
            0xFFFFFFFF80000000_u64 as i64
        );
        assert_eq!(
            zigzag::decode_64(0xFFFFFFFFFFFFFFFE),
            0x7FFFFFFFFFFFFFFF_u64 as i64
        );
        assert_eq!(
            zigzag::decode_64(0xFFFFFFFFFFFFFFFF),
            0x8000000000000000_u64 as i64
        );
    }
}