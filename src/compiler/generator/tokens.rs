use syn::token::*;

use proc_macro2::Span;

pub fn span() -> Span {
    return Span::call_site();
}

pub fn semi_token() -> Semi {
    return Semi {
        spans: [span()],
    };
}

pub fn pub_token() -> Pub {
    return Pub {
        span: span(),
    };
}

pub fn mut_token() -> Mut {
    return Mut {
        span: span(),
    };
}

pub fn mod_token() -> Mod {
    return Mod {
        span: span(),
    };
}

pub fn use_token() -> Use {
    return Use {
        span: span(),
    };
}

pub fn fn_token() -> Fn {
    return Fn {
        span: span(),
    };
}

pub fn dyn_token() -> Dyn {
    return Dyn {
        span: span(),
    };
}

pub fn paren_token() -> Paren {
    return Paren {
        span: span(),
    };
}

pub fn const_token() -> Const {
    return Const {
        span: span(),
    };
}

pub fn colon_token() -> Colon {
    return Colon {
        spans: [span()],
    };
}

pub fn colon2_token() -> Colon2 {
    return Colon2 {
        spans: [span(), span()],
    };
}

pub fn brace_token() -> Brace {
    return Brace {
        span: span(),
    };
}

pub fn bracket_token() -> Bracket {
    return Bracket {
        span: span(),
    };
}

pub fn eq_token() -> Eq {
    return Eq {
        spans: [span()],
    };
}

pub fn add_token() -> Add {
    return Add {
        spans: [span()],
    };
}

pub fn sub_token() -> Sub {
    return Sub {
        spans: [span()],
    };
}

pub fn star_token() -> Star {
    return Star {
        spans: [span()],
    };
}

pub fn div_token() -> Div {
    return Div {
        spans: [span()],
    };
}

pub fn rem_token() -> Rem {
    return Rem {
        spans: [span()],
    };
}

pub fn and_and_token() -> AndAnd {
    return AndAnd {
        spans: [span(), span()],
    };
}

pub fn or_or_token() -> OrOr {
    return OrOr {
        spans: [span(), span()],
    };
}

pub fn eq_eq_token() -> EqEq {
    return EqEq {
        spans: [span(), span()],
    };
}

pub fn lt_token() -> Lt {
    return Lt {
        spans: [span()],
    };
}

pub fn le_token() -> Le {
    return Le {
        spans: [span(), span()],
    };
}

pub fn ne_token() -> Ne {
    return Ne {
        spans: [span(), span()],
    };
}

pub fn ge_token() -> Ge {
    return Ge {
        spans: [span(), span()],
    };
}

pub fn gt_token() -> Gt {
    return Gt {
        spans: [span()],
    };
}

pub fn add_eq_token() -> AddEq {
    return AddEq {
        spans: [span(), span()],
    };
}

pub fn sub_eq_token() -> SubEq {
    return SubEq {
        spans: [span(), span()],
    };
}

pub fn mul_eq_token() -> MulEq {
    return MulEq {
        spans: [span(), span()],
    };
}

pub fn div_eq_token() -> DivEq {
    return DivEq {
        spans: [span(), span()],
    };
}

pub fn rem_eq_token() -> RemEq {
    return RemEq {
        spans: [span(), span()],
    };
}

pub fn r_arrow_token() -> RArrow {
    return RArrow {
        spans: [span(), span()],
    };
}

pub fn comma_token() -> Comma {
    return Comma {
        spans: [span()],
    };
}

pub fn impl_token() -> Impl {
    return Impl {
        span: span(),
    };
}

pub fn underscore_token() -> Underscore {
    return Underscore {
        spans: [span()],
    };
}

pub fn and_token() -> And {
    return And {
        spans: [span()],
    };
}

pub fn async_token() -> Async {
    return Async {
        span: span(),
    };
}

pub fn await_token() -> Await {
    return Await {
        span: span(),
    };
}

pub fn move_token() -> Move {
    return Move {
        span: span(),
    };
}

pub fn dot_token() -> Dot {
    return Dot {
        spans: [span()],
    };
}

pub fn dot2_token() -> Dot2 {
    return Dot2 {
        spans: [span(), span()],
    };
}

pub fn dot_dot_eq_token() -> DotDotEq {
    return DotDotEq {
        spans: [span(), span(), span()],
    };
}

pub fn as_token() -> As {
    return As {
        span: span(),
    };
}

pub fn or_token() -> Or {
    return Or {
        spans: [span()],
    };
}

pub fn continue_token() -> Continue {
    return Continue {
        span: span(),
    };
}

pub fn break_token() -> Break {
    return Break {
        span: span(),
    };
}

pub fn for_token() -> For {
    return For {
        span: span(),
    };
}

pub fn in_token() -> In {
    return In {
        span: span(),
    };
}

pub fn if_token() -> If {
    return If {
        span: span(),
    };
}

pub fn else_token() -> Else {
    return Else {
        span: span(),
    };
}

pub fn let_token() -> Let {
    return Let {
        span: span(),
    };
}

pub fn loop_token() -> Loop {
    return Loop {
        span: span(),
    };
}

pub fn bang_token() -> Bang {
    return Bang {
        spans: [span()],
    };
}

pub fn match_token() -> Match {
    return Match {
        span: span(),
    };
}

pub fn fat_arrow_token() -> FatArrow {
    return FatArrow {
        spans: [span(), span()],
    };
}

pub fn return_token() -> Return {
    return Return {
        span: span(),
    };
}

pub fn question_token() -> Question {
    return Question {
        spans: [span()],
    };
}

pub fn while_token() -> While {
    return While {
        span: span(),
    };
}

pub fn ref_token() -> Ref {
    return Ref {
        span: span(),
    };
}

pub fn enum_token() -> Enum {
    return Enum {
        span: span(),
    };
}

pub fn self_token() -> SelfValue {
    return SelfValue {
        span: span(),
    };
}

pub fn type_token() -> Type {
    return Type {
        span: span(),
    };
}

pub fn struct_token() -> Struct {
    return Struct {
        span: span(),
    };
}

pub fn trait_token() -> Trait {
    return Trait {
        span: span(),
    };
}

