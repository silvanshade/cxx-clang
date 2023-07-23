pub(crate) mod decl_context;
pub(crate) mod declarator_decl;
pub(crate) mod field_decl;
pub(crate) mod function_decl;
pub(crate) mod named_decl;
pub(crate) mod objc_container_decl;
pub(crate) mod objc_interface_decl;
pub(crate) mod objc_method_decl;
pub(crate) mod objc_protocol_decl;
pub(crate) mod record_decl;
pub(crate) mod tag_decl;
pub(crate) mod type_decl;
pub(crate) mod typedef_decl;
pub(crate) mod typedef_name_decl;
pub(crate) mod value_decl;

use crate::{
    ffi::clang::ast::decl::{
        field_decl::FieldDecl,
        function_decl::FunctionDecl,
        named_decl::NamedDecl,
        objc_interface_decl::ObjcInterfaceDecl,
        objc_method_decl::ObjcMethodDecl,
        objc_protocol_decl::ObjcProtocolDecl,
        record_decl::RecordDecl,
        type_decl::TypeDecl,
        typedef_decl::TypedefDecl,
        typedef_name_decl::TypedefNameDecl,
    },
    gen::clang::ast::decl,
};

pub use crate::{abi::clang::ast::decl::Decl, gen::clang::ast::decl::Kind};

impl<'ctx> Decl<'ctx> {
    #[inline]
    pub fn get_kind(&self) -> decl::Kind {
        decl::get_kind(self)
    }
}

impl<'ctx> Decl<'ctx> {
    #[inline]
    pub fn cast_as_field_decl(&self) -> Option<&FieldDecl<'ctx>> {
        let ptr = decl::cast_as_field_decl(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }

    #[inline]
    pub fn cast_as_function_decl(&self) -> Option<&FunctionDecl<'ctx>> {
        let ptr = decl::cast_as_function_decl(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }

    #[inline]
    pub fn cast_as_named_decl(&self) -> Option<&NamedDecl<'ctx>> {
        let ptr = decl::cast_as_named_decl(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }

    #[inline]
    pub fn cast_as_objc_interface_decl(&self) -> Option<&ObjcInterfaceDecl<'ctx>> {
        let ptr = decl::cast_as_objc_interface_decl(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }

    #[inline]
    pub fn cast_as_objc_method_decl(&self) -> Option<&ObjcMethodDecl<'ctx>> {
        let ptr = decl::cast_as_objc_method_decl(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }

    #[inline]
    pub fn cast_as_objc_protocol_decl(&self) -> Option<&ObjcProtocolDecl<'ctx>> {
        let ptr = decl::cast_as_objc_protocol_decl(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }

    #[inline]
    pub fn cast_as_record_decl(&self) -> Option<&RecordDecl<'ctx>> {
        let ptr = decl::cast_as_record_decl(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }

    #[inline]
    pub fn cast_as_type_decl(&self) -> Option<&TypeDecl<'ctx>> {
        let ptr = decl::cast_as_type_decl(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }

    #[inline]
    pub fn cast_as_typedef_decl(&self) -> Option<&TypedefDecl<'ctx>> {
        let ptr = decl::cast_as_typedef_decl(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }

    #[inline]
    pub fn cast_as_typedef_name_decl(&self) -> Option<&TypedefNameDecl<'ctx>> {
        let ptr = decl::cast_as_typedef_name_decl(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }
}
