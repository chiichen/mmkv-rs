use cxx::{type_id, ExternType};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::os::raw::{c_char, c_void};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct StringView<'a> {
    repr: MaybeUninit<[*const c_void; 2]>,
    borrow: PhantomData<&'a [c_char]>,
}

unsafe impl<'a> ExternType for StringView<'a> {
    type Id = type_id!("std::string_view");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("include/string_view.h");

        #[namespace = "std"]
        #[cxx_name = "string_view"]
        type StringView<'a> = crate::base::StringView<'a>;

        fn string_view_from_str<'a>(s: &'a str) -> StringView<'a>;
        fn string_view_as_bytes<'a>(s: StringView<'a>) -> &'a [c_char];
    }
}

impl<'a> StringView<'a> {
    pub fn new(s: &'a str) -> Self {
        ffi::string_view_from_str(s)
    }

    pub fn as_bytes(self) -> &'a [c_char] {
        ffi::string_view_as_bytes(self)
    }
}

impl<'a> Deref for StringView<'a> {
    type Target = [c_char];
    fn deref(&self) -> &Self::Target {
        self.as_bytes()
    }
}