#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[repr(C, align(8))]
pub struct Module<'ctx> {
    _layout: [u8; 2512],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _pinned: ::core::marker::PhantomPinned,
    _lifetimes: ::core::marker::PhantomData<(&'ctx (),)>,
}
unsafe impl<'ctx> ::cxx::ExternType for Module<'ctx> {
    type Id = ::cxx::type_id!("cxx_clang::clang::basic::module::Module");
    type Kind = ::cxx::kind::Opaque;
}
impl<'ctx> ::core::ops::Drop for Module<'ctx> {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self::ffi::cxx_destruct(self);
        }
    }
}
#[cfg(feature = "debug")]
impl<'ctx> ::core::fmt::Debug for Module<'ctx> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Module").finish()
    }
}
unsafe impl<'ctx> ::cxx_memory::CopyNew for Module<'ctx> {
    #[inline]
    unsafe fn copy_new(that: &Self, this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>) {
        let this = this.get_unchecked_mut().as_mut_ptr();
        self::ffi::cxx_copy_new(this, that)
    }
}
unsafe impl<'ctx> ::cxx_memory::MoveNew for Module<'ctx> {
    #[inline]
    unsafe fn move_new(
        that: ::core::pin::Pin<::cxx_memory::MoveRef<'_, Self>>,
        this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>,
    ) {
        let this = this.get_unchecked_mut().as_mut_ptr();
        let that = &mut *::core::pin::Pin::into_inner_unchecked(that);
        self::ffi::cxx_move_new(this, that)
    }
}
#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "cxx_clang::clang::basic::module"]
    unsafe extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/Basic/Module.hxx");
        #[cxx_name = "Module"]
        #[allow(unused)]
        type Module<'ctx> = super::Module<'ctx>;
        unsafe fn cxx_copy_new<'ctx>(This: *mut Module<'ctx>, that: &Module<'ctx>);
        unsafe fn cxx_move_new<'ctx>(This: *mut Module<'ctx>, that: *mut Module<'ctx>);
        unsafe fn cxx_destruct<'ctx>(This: *mut Module<'ctx>);
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<Module<'static>>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<Module<'static>>(), 2512)
        }
    }
}
