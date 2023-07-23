mod abi;
mod ffi {
    pub(crate) mod clang {
        pub(crate) mod ast {
            pub(crate) mod ast_context;
            pub(crate) mod decl;
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
            pub mod decl_context {
                pub use crate::ffi::clang::ast::decl::decl_context::DeclContext;
            }
            pub mod declarator_decl {
                pub use crate::ffi::clang::ast::decl::declarator_decl::DeclaratorDecl;
            }
            pub mod field_decl {
                pub use crate::ffi::clang::ast::decl::field_decl::FieldDecl;
            }
            pub mod function_decl {
                pub use crate::ffi::clang::ast::decl::function_decl::FunctionDecl;
            }
            pub mod named_decl {
                pub use crate::ffi::clang::ast::decl::named_decl::NamedDecl;
            }
            pub mod objc_container_decl {
                pub use crate::ffi::clang::ast::decl::objc_container_decl::ObjcContainerDecl;
            }
            pub mod objc_interface_decl {
                pub use crate::ffi::clang::ast::decl::objc_interface_decl::ObjcInterfaceDecl;
            }
            pub mod objc_method_decl {
                pub use crate::ffi::clang::ast::decl::objc_method_decl::ObjcMethodDecl;
            }
            pub mod objc_protocl_decl {
                pub use crate::ffi::clang::ast::decl::objc_protocol_decl::ObjcProtocolDecl;
            }
            pub mod record_decl {
                pub use crate::ffi::clang::ast::decl::record_decl::RecordDecl;
            }
            pub mod tag_decl {
                pub use crate::ffi::clang::ast::decl::tag_decl::TagDecl;
            }
            pub mod type_decl {
                pub use crate::ffi::clang::ast::decl::type_decl::TypeDecl;
            }
            pub mod typedef_decl {
                pub use crate::ffi::clang::ast::decl::typedef_decl::TypedefDecl;
            }
            pub mod typedef_name_decl {
                pub use crate::ffi::clang::ast::decl::typedef_name_decl::TypedefNameDecl;
            }
            pub mod value_decl {
                pub use crate::ffi::clang::ast::decl::value_decl::ValueDecl;
            }
            pub use crate::ffi::clang::ast::decl::{Decl, Kind};
        }
    }
    pub mod basic {
        pub mod module {
            pub use crate::ffi::clang::basic::module::Module;
        }
    }
    pub mod lex {
        pub mod macro_info {
            pub mod module_macro {
                pub use crate::ffi::clang::lex::macro_info::module_macro::ModuleMacro;
            }
            pub use crate::ffi::clang::lex::macro_info::MacroInfo;
        }
    }
    pub use crate::ffi::clang::{
        ast::{
            ast_context::AstContext,
            decl::{
                decl_context::DeclContext,
                declarator_decl::DeclaratorDecl,
                field_decl::FieldDecl,
                function_decl::FunctionDecl,
                named_decl::NamedDecl,
                objc_container_decl::ObjcContainerDecl,
                objc_interface_decl::ObjcInterfaceDecl,
                objc_method_decl::ObjcMethodDecl,
                objc_protocol_decl::ObjcProtocolDecl,
                record_decl::RecordDecl,
                tag_decl::TagDecl,
                type_decl::TypeDecl,
                typedef_decl::TypedefDecl,
                typedef_name_decl::TypedefNameDecl,
                value_decl::ValueDecl,
                Decl,
                Kind as DeclKind,
            },
        },
        basic::module::Module,
        lex::macro_info::{module_macro::ModuleMacro, MacroInfo},
    };
}
