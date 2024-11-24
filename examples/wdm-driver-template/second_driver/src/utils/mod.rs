
use wdk_sys::UNICODE_STRING;
use alloc::vec::Vec;
use wdk_sys::ntddk::RtlInitUnicodeStringEx;

pub trait ToUnicodeString {
    fn to_unicode(&self) -> UNICODE_STRING;
}

impl ToUnicodeString for &str {
    fn to_unicode(&self) -> UNICODE_STRING {



        let mut unicode_string = UNICODE_STRING {
            Length: 0,
            MaximumLength: 0,
            Buffer: core::ptr::null_mut(),
        };

        let mut buffer: Vec<u16> = self.encode_utf16().chain(Some(0)).collect();
        

        unsafe {
            RtlInitUnicodeStringEx(&mut unicode_string, buffer.as_ptr());
        }

        unicode_string
    }
}