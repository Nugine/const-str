//! Rewritten from <https://github.com/uuid-rs/uuid/blob/9d06072aa5219f317097ab7bf4ea44caddbc409a/shared/parser.rs>

#[derive(Debug, PartialEq, Eq)]
pub struct Error(ErrorKind);

#[derive(Debug, PartialEq, Eq)]
enum ErrorKind {
    InvalidLength,
    InvalidCharacter,
}

const fn invalid_length() -> Error {
    Error(ErrorKind::InvalidLength)
}

const fn invalid_character() -> Error {
    Error(ErrorKind::InvalidCharacter)
}

const fn strip_prefix<'a>(s: &'a str, prefix: &str) -> Option<&'a [u8]> {
    if prefix.len() > s.len() {
        return None;
    }
    let s = s.as_bytes();
    let prefix = prefix.as_bytes();
    let mut i = 0;
    while i < prefix.len() {
        if prefix[i] != s[i] {
            return None;
        }
        i += 1
    }
    Some(crate::bytes::advance(s, prefix.len()))
}

const fn try_parse_uuid(s: &str) -> Result<[u8; 16], Error> {
    match s.len() {
        // URN prefixed UUID
        45 => match strip_prefix(s, "urn:uuid:") {
            Some(s) => parse_uuid_hyphenated(s),
            None => Err(invalid_length()),
        },
        // Microsoft GUID
        38 => match s.as_bytes() {
            [b'{', xs @ .., b'}'] => parse_uuid_hyphenated(xs),
            _ => Err(invalid_length()),
        },
        // hyphenated UUID
        36 => parse_uuid_hyphenated(s.as_bytes()),
        // simple UUID
        32 => parse_uuid_simple(s.as_bytes()),
        _ => Err(invalid_length()),
    }
}

type Table = [u8; 256];

const fn generate_lookup_table() -> Table {
    let mut buf: [u8; 256] = [0; 256];
    let mut i: u8 = 0;
    loop {
        buf[i as usize] = match i {
            b'0'..=b'9' => i - b'0',
            b'a'..=b'f' => i - b'a' + 10,
            b'A'..=b'F' => i - b'A' + 10,
            _ => 0xff,
        };
        if i == 255 {
            return buf;
        }
        i += 1
    }
}

const HEX_TABLE: Table = generate_lookup_table();

const fn generate_shl4_table() -> Table {
    let mut buf: [u8; 256] = [0; 256];
    let mut i: u8 = 0;
    loop {
        buf[i as usize] = i.wrapping_shl(4);
        if i == 255 {
            return buf;
        }
        i += 1;
    }
}

const SHL4_TABLE: Table = generate_shl4_table();

const fn parse_uuid_simple(s: &[u8]) -> Result<[u8; 16], Error> {
    constfn_assert!(s.len() == 32);
    let mut buf: [u8; 16] = [0; 16];
    let mut i = 0;
    while i < 16 {
        let h1 = HEX_TABLE[s[i * 2] as usize];
        let h2 = HEX_TABLE[s[i * 2 + 1] as usize];
        if h1 | h2 == 0xff {
            return Err(invalid_character());
        }
        buf[i] = SHL4_TABLE[h1 as usize] | h2;
        i += 1;
    }
    Ok(buf)
}

const fn parse_uuid_hyphenated(s: &[u8]) -> Result<[u8; 16], Error> {
    constfn_assert!(s.len() == 36);

    // b'-' is 0x2d
    let x = u32::from_ne_bytes([s[8], s[13], s[18], s[23]]);
    if x != 0x2d2d2d2d {
        return Err(invalid_character());
    }

    let positions: [u8; 8] = [0, 4, 9, 14, 19, 24, 28, 32];
    let mut buf: [u8; 16] = [0; 16];
    let mut j = 0;
    while j < 8 {
        let i = positions[j];
        let h1 = HEX_TABLE[s[i as usize] as usize];
        let h2 = HEX_TABLE[s[(i + 1) as usize] as usize];
        let h3 = HEX_TABLE[s[(i + 2) as usize] as usize];
        let h4 = HEX_TABLE[s[(i + 3) as usize] as usize];
        if h1 | h2 | h3 | h4 == 0xff {
            return Err(invalid_character());
        }
        buf[j * 2] = SHL4_TABLE[h1 as usize] | h2;
        buf[j * 2 + 1] = SHL4_TABLE[h3 as usize] | h4;
        j += 1;
    }

    Ok(buf)
}

#[test]
fn test_parse_uuid() {
    const A1: &str = "67e55044 10b1 426f 9247 bb680e5fe0c8";
    const A2: &str = "00000000000000000000000000000000";

    const OK: &[(&str, &str)] = &[
        (A1, "67e55044-10b1-426f-9247-bb680e5fe0c8"),
        (A1, "67e5504410b1426f9247bb680e5fe0c8"),
        (A1, "{67e55044-10b1-426f-9247-bb680e5fe0c8}"),
        (A1, "urn:uuid:67e55044-10b1-426f-9247-bb680e5fe0c8"),
        (A2, "00000000000000000000000000000000"),
        (A2, "00000000-0000-0000-0000-000000000000"),
        (
            "01020304 1112 2122 3132 414243444546",
            "01020304-1112-2122-3132-414243444546",
        ),
        (
            "F9168C5E CEB2 4faa B6BF 329BF39FA1E4",
            "F9168C5E-CEB2-4faa-B6BF-329BF39FA1E4",
        ),
        (
            "6d93bade bd9f 4e13 8914 9474e1e3567b",
            "{6d93bade-bd9f-4e13-8914-9474e1e3567b}",
        ),
    ];

    for &(expected, input) in OK {
        let expected_bytes: [u8; 16] = super::HexBytes(expected).const_eval();
        assert_eq!(try_parse_uuid(input).unwrap(), expected_bytes);
    }

    const B1: Error = invalid_length();
    const B2: Error = invalid_character();

    const ERR: &[(Error, &str)] = &[
        (B1, ""),
        (B1, "!"),
        (B1, "F9168C5E-CEB2-4faa-B6BF-329BF39FA1E45"),
        (B1, "F9168C5E-CEB2-4faa-BBF-329BF39FA1E4"),
        (B1, "F9168C5E-CEB2-4faa"),
        (B1, "{F9168C5E-CEB2-4faa9B6BFF329BF39FA1E41"),
        (B1, "67e5504410b1426f9247bb680e5fe0c"),
        (B1, "67e5504410b1426f9247bb680e5fe0c88"),
        (B1, "67e5504410b1426f9247bb680e5fe0cg8"),
        (B1, "{00000000000000000000000000000000}"),
        (B1, "67e5504410b1426f9247bb680e5fe0c"),
        (B2, "F9168C5E-CEB2-4faa-B6BF1-02BF39FA1E4"),
        (B2, "231231212212423424324323477343246663"),
        (B2, "01020304-1112-2122-3132-41424344"),
        (B2, "F9168C5E-CEB2-4faa-BGBF-329BF39FA1E4"),
        (B2, "F9168C5E-CEB2F4faaFB6BFF329BF39FA1E4"),
        (B2, "F9168C5E-CEB2-4faaFB6BFF329BF39FA1E4"),
        (B2, "F9168C5E-CEB2-4faa-B6BFF329BF39FA1E4"),
        (B2, "F9168C5E-CEB2-4faaXB6BFF329BF39FA1E4"),
        (B2, "67e5504410b1426%9247bb680e5fe0c8"),
        (B2, "67e550X410b1426f9247bb680e5fe0cd"),
        (B2, "67e550-4105b1426f9247bb680e5fe0c"),
        (B2, "F9168C5E-CEB-24fa-eB6BFF32-BF39FA1E4"),
    ];

    for &(ref expected, input) in ERR {
        assert_eq!(&try_parse_uuid(input).unwrap_err(), expected);
    }
}

pub const fn parse_uuid(s: &str) -> uuid::Uuid {
    match try_parse_uuid(s) {
        Ok(b) => uuid::Uuid::from_bytes(b),
        Err(_) => constfn_panic!("failed to parse uuid"),
    }
}

/// Converts a string slice to [`Uuid`][uuid::Uuid]
///
/// This macro requires the feature `uuid`.
///
/// # Examples
///
/// ```
/// use uuid::Uuid;
/// use const_str::uuid;
///
/// pub const SCHEMA_ATTR_CLASS: Uuid = uuid!("00000000-0000-0000-0000-ffff00000000");
/// pub const SCHEMA_ATTR_UUID: Uuid = uuid!("00000000-0000-0000-0000-ffff00000001");
/// pub const SCHEMA_ATTR_NAME: Uuid = uuid!("00000000-0000-0000-0000-ffff00000002");
///
/// pub const URN_UUID_TEXT: &str = "urn:uuid:F9168C5E-CEB2-4faa-B6BF-329BF39FA1E4";
/// pub const URN_UUID: Uuid = uuid!(URN_UUID_TEXT);
/// ```
#[macro_export]
macro_rules! uuid {
    ($s: expr) => {{
        const INPUT: &str = $s;
        $crate::__ctfe::parse_uuid(INPUT)
    }};
}
