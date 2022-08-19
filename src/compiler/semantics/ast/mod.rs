mod ast_enum;
mod ast_expr;
mod ast_fn;
mod ast_generics;
mod ast_impl;
mod ast_lazy_static_consts;
mod ast_mod;
mod ast_pattern;
mod ast_statement;
mod ast_static_consts;
mod ast_struct;
mod ast_trait;
mod ast_type;
mod ast_use;

pub use ast_enum::*;
pub use ast_expr::*;
pub use ast_fn::*;
pub use ast_generics::*;
pub use ast_impl::*;
pub use ast_lazy_static_consts::*;
pub use ast_mod::*;
pub use ast_pattern::*;
pub use ast_statement::*;
pub use ast_static_consts::*;
pub use ast_struct::*;
pub use ast_trait::*;
pub use ast_type::*;
pub use ast_use::*;

/*
 * TODO
 *
 * SemanticAST is what will define things like:
 *  - Explicit main function
 *  - Ownership vs borrowing
 *  - Lifetimes
 *  - RC and GC
 *  - Applying defaults
 *  - Can [blank] be known at build-time
 *  - etc
 *
 * SemanticAST is much closer to Rust implementation. Logic from Ferrum to Rust is mostly done
 * here; the generator just translates our SemanticAST to the AST in `syn` crate.
 */

// SemanticAST takes heavy inspiration from syn crate: https://docs.rs/syn/latest/syn/struct.File.html

#[derive(Clone, Debug, PartialEq)]
pub struct SemanticAST {
    pub mods: Vec<Mod>,
    pub uses: Vec<Use>,
    pub static_consts: Vec<StaticConst>,
    pub lazy_static_consts: Vec<LazyStaticConst>,
    pub items: Vec<Item>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Item {
    Struct(ItemStruct),
    Impl(ItemImpl),
    Enum(ItemEnum),
    Trait(ItemTrait),
    Fn(ItemFn),
}

