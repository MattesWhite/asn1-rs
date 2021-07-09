use crate::asn1_string;
use crate::{Error, Result};
use alloc::string::String;

asn1_string!(UniversalString);

impl<'a> UniversalString<'a> {
    fn test_string_charset(i: &[u8]) -> Result<()> {
        if !i.iter().all(u8::is_ascii) {
            return Err(Error::StringInvalidCharset);
        }
        Ok(())
    }
}
