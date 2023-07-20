#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[repr(C, align(8))]
pub struct AstContext<'ctx> {
    _layout: [u8; 21000],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _pinned: ::core::marker::PhantomPinned,
    _lifetimes: ::core::marker::PhantomData<(&'ctx (),)>,
}
unsafe impl<'ctx> ::cxx::ExternType for AstContext<'ctx> {
    type Id = ::cxx::type_id!("cxx_clang::clang::ast::ast_context::ASTContext");
    type Kind = ::cxx::kind::Opaque;
}
impl<'ctx> ::core::ops::Drop for AstContext<'ctx> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self::ffi::cxx_destruct(self);
        }
    }
}
#[cfg(feature = "debug")]
impl<'ctx> ::core::fmt::Debug for AstContext<'ctx> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AstContext").finish()
    }
}
#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "cxx_clang::clang::ast::ast_context"]
    unsafe extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/ASTContext.hxx");
        #[cxx_name = "ASTContext"]
        #[allow(unused)]
        type AstContext<'ctx> = super::AstContext<'ctx>;
        unsafe fn cxx_destruct<'ctx>(This: *mut AstContext<'ctx>);
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<AstContext<'static>>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<AstContext<'static>>(), 21000)
        }
    }
}
