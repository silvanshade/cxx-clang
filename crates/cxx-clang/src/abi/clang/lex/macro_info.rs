#![doc = r" NOTE: This module is auto-generated and should not be edited."]
pub(crate) mod module_macro;
#[repr(C, align(8))]
pub struct MacroInfo<'ctx> {
    _layout: [u8; 40],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _pinned: ::core::marker::PhantomPinned,
    _lifetimes: ::core::marker::PhantomData<(&'ctx (),)>,
}
unsafe impl<'ctx> ::cxx::ExternType for MacroInfo<'ctx> {
    type Id = ::cxx::type_id!("cxx_clang::clang::lex::macro_info::MacroInfo");
    type Kind = ::cxx::kind::Opaque;
}
#[cfg(feature = "debug")]
impl<'ctx> ::core::fmt::Debug for MacroInfo<'ctx> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MacroInfo").finish()
    }
}
#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "cxx_clang::clang::lex::macro_info"]
    unsafe extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/Lex/MacroInfo.hxx");
        #[cxx_name = "MacroInfo"]
        #[allow(unused)]
        type MacroInfo<'ctx> = super::MacroInfo<'ctx>;
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<MacroInfo<'static>>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<MacroInfo<'static>>(), 40)
        }
    }
}
