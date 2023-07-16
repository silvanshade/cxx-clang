mod abi;
mod ffi {
    pub(crate) mod clang {
        pub(crate) mod ast {
            pub(crate) mod ast_context;
            pub(crate) mod decl;
            pub(crate) mod named_decl;
        }
        pub(crate) mod basic {
            pub(crate) mod module;
        }
        pub(crate) mod lex {
            pub(crate) mod macro_info;
        }
    }
}
mod gen {
    pub(crate) mod clang {
        pub(crate) mod ast {
            pub(crate) mod ast_context;
            pub(crate) mod decl;
            pub(crate) mod named_decl;
        }
        pub(crate) mod basic {
            pub(crate) mod module;
        }
        pub(crate) mod lex {
            pub(crate) mod macro_info;
        }
    }
}

pub mod clang {
    pub mod ast {
        pub mod ast_context {
            pub use crate::ffi::clang::ast::ast_context::AstContext;
        }
        pub mod decl {
            pub use crate::ffi::clang::ast::decl::Decl;
        }
        pub mod named_decl {
            pub use crate::ffi::clang::ast::named_decl::NamedDecl;
        }
    }
    pub mod basic {
        pub mod module {
            pub use crate::ffi::clang::basic::module::Module;
        }
    }
    pub mod lex {
        pub mod macro_info {
            pub use crate::ffi::clang::lex::macro_info::MacroInfo;
        }
    }
    pub use crate::ffi::clang::{
        ast::{ast_context::AstContext, decl::Decl, named_decl::NamedDecl},
        basic::module::Module,
        lex::macro_info::MacroInfo,
    };
}
