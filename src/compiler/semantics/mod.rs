use super::parser::{AST, *};

use proc_macro2::Span;

use anyhow::Result;

use core::str::FromStr;

pub fn translate(ast: AST) -> Result<syn::File> {
    let mut has_explicit_main = None;

    for node in ast.nodes.into_iter() {
        match node {
            RootNode::Statement(_) => {
                if let Some(has_main) = has_explicit_main {
                    if has_main {
                        panic!("either top-level statements OR main function");
                    }
                } else {
                    has_explicit_main = Some(false);
                }
            },
            RootNode::Definition(Definition::Function(def_fn)) if def_fn.signature.name.as_str() == "main" => {
                if let Some(has_main) = has_explicit_main {
                    if has_main {
                        panic!("only 1 main function allowed");
                    } else {
                        panic!("either top-level statements OR main function");
                    }
                } else {
                    has_explicit_main = Some(true);
                }
            },
            _ => {},
        }
    }

    if has_explicit_main.is_none() {
        has_explicit_main = Some(false);
    }

    println!("has_explicit_main: {has_explicit_main:?}");

    let file = syn::File {
        shebang: None,
        attrs: vec![],
        items: vec![
            syn::Item::Fn(syn::ItemFn {
                attrs: vec![],
                vis: syn::Visibility::Public(syn::VisPublic {
                    pub_token: syn::token::Pub {
                        span: Span::call_site(),
                    },
                }),
                sig: syn::Signature {
                    constness: None,
                    asyncness: None,
                    unsafety: None,
                    abi: None,
                    fn_token: syn::token::Fn {
                        span: Span::call_site(),
                    },
                    ident: syn::Ident::new("main", Span::call_site()),
                    generics: syn::Generics {
                        gt_token: None,
                        lt_token: None,
                        where_clause: None,
                        params: syn::punctuated::Punctuated::new(),
                    },
                    paren_token: syn::token::Paren {
                        span: Span::call_site(),
                    },
                    inputs: syn::punctuated::Punctuated::new(),
                    variadic: None,
                    output: syn::ReturnType::Default,
                },
                block: Box::new(syn::Block {
                    brace_token: syn::token::Brace {
                        span: Span::call_site(),
                    },
                    stmts: vec![
                        syn::Stmt::Semi(
                            syn::Expr::Macro(syn::ExprMacro {
                                attrs: vec![],
                                mac: syn::Macro {
                                    path: syn::Path {
                                        leading_colon: None,
                                        segments: {
                                            let mut punc = syn::punctuated::Punctuated::new();
                                            punc.push_value(syn::PathSegment {
                                                ident: syn::Ident::new("println", Span::call_site()),
                                                arguments: syn::PathArguments::None,
                                           });
                                            punc
                                        },
                                    },
                                    bang_token: syn::token::Bang {
                                        spans: [Span::call_site()],
                                    },
                                    delimiter: syn::MacroDelimiter::Paren(syn::token::Paren {
                                        span: Span::call_site(),
                                    }),
                                    tokens: proc_macro2::TokenStream::from_str("\"hello world\"").unwrap(),
                                },
                            }),
                            syn::token::Semi {
                                spans: [Span::call_site()],
                            },
                        ),
                    ],
                }),
            }),
        ],
    };

    return Ok(file);
}

