use crate::gen::clang::ast::decl;

pub use crate::abi::clang::ast::decl::Decl;

impl<'ctx> Decl<'ctx> {
    #[inline]
    pub fn get_kind(&self) -> decl::Kind {
        decl::get_kind(self)
    }
}
