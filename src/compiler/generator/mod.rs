use super::semantics::*;

use core::str::FromStr;

use quote::ToTokens;

use proc_macro2::Span;

use anyhow::Result;

pub fn generate_rust(ast: SemanticAST) -> Result<String> {
    /* TODO
     * syn's AST has many complex pieces within.
     * Might be worth trying to build string ourselves.
     */

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

    return Ok(file.into_token_stream().to_string());
}

