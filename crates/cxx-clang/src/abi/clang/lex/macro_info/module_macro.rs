#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct ModuleMacro<'ctx> {
    _layout: [u8; 40],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _lifetimes: ::core::marker::PhantomData<(&'ctx (),)>,
}
unsafe impl<'ctx> ::cxx::ExternType for ModuleMacro<'ctx> {
    type Id = ::cxx::type_id!("cxx_clang::clang::lex::macro_info::module_macro::ModuleMacro");
    type Kind = ::cxx::kind::Trivial;
}
#[cfg(feature = "debug")]
impl<'ctx> ::core::fmt::Debug for ModuleMacro<'ctx> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ModuleMacro").finish()
    }
}
unsafe impl<'ctx> ::cxx_memory::CopyNew for ModuleMacro<'ctx> {
    #[inline]
    unsafe fn copy_new(that: &Self, this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>) {
        let this = this.get_unchecked_mut().as_mut_ptr();
        self::ffi::cxx_copy_new(this, that)
    }
}
unsafe impl<'ctx> ::cxx_memory::MoveNew for ModuleMacro<'ctx> {
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
    #[namespace = "cxx_clang::clang::lex::macro_info::module_macro"]
    unsafe extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/Lex/MacroInfo/ModuleMacro.hxx");
        #[cxx_name = "ModuleMacro"]
        #[allow(unused)]
        type ModuleMacro<'ctx> = super::ModuleMacro<'ctx>;
        unsafe fn cxx_copy_new<'ctx>(This: *mut ModuleMacro<'ctx>, that: &ModuleMacro<'ctx>);
        unsafe fn cxx_move_new<'ctx>(This: *mut ModuleMacro<'ctx>, that: *mut ModuleMacro<'ctx>);
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<ModuleMacro<'static>>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<ModuleMacro<'static>>(), 40)
        }
        :: static_assertions :: assert_impl_all ! (ModuleMacro < 'static > : :: core :: marker :: Copy);
        :: static_assertions :: assert_impl_all ! (ModuleMacro < 'static > : :: core :: marker :: Unpin);
    }
}
