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

    // let file = syn::File {
    //     shebang: None,
    //     attrs: vec![],
    //     items: vec![
    //         syn::Item::Fn(syn::ItemFn {
    //             attrs: vec![],
    //             vis: syn::Visibility::Public(syn::VisPublic {
    //                 pub_token: syn::token::Pub {
    //                     span: span(),
    //                 },
    //             }),
    //             sig: syn::Signature {
    //                 constness: None,
    //                 asyncness: None,
    //                 unsafety: None,
    //                 abi: None,
    //                 fn_token: syn::token::Fn {
    //                     span: span(),
    //                 },
    //                 ident: syn::Ident::new("main", span()),
    //                 generics: syn::Generics {
    //                     gt_token: None,
    //                     lt_token: None,
    //                     where_clause: None,
    //                     params: syn::punctuated::Punctuated::new(),
    //                 },
    //                 paren_token: syn::token::Paren {
    //                     span: span(),
    //                 },
    //                 inputs: syn::punctuated::Punctuated::new(),
    //                 variadic: None,
    //                 output: syn::ReturnType::Default,
    //             },
    //             block: Box::new(syn::Block {
    //                 brace_token: syn::token::Brace {
    //                     span: span(),
    //                 },
    //                 stmts: vec![
    //                     syn::Stmt::Semi(
    //                         syn::Expr::Macro(syn::ExprMacro {
    //                             attrs: vec![],
    //                             mac: syn::Macro {
    //                                 path: syn::Path {
    //                                     leading_colon: None,
    //                                     segments: {
    //                                         let mut punc = syn::punctuated::Punctuated::new();
    //                                         punc.push_value(syn::PathSegment {
    //                                             ident: syn::Ident::new("println", Span::call_site()),
    //                                             arguments: syn::PathArguments::None,
    //                                        });
    //                                         punc
    //                                     },
    //                                 },
    //                                 bang_token: syn::token::Bang {
    //                                     spans: [Span::call_site()],
    //                                 },
    //                                 delimiter: syn::MacroDelimiter::Paren(syn::token::Paren {
    //                                     span: Span::call_site(),
    //                                 }),
    //                                 tokens: proc_macro2::TokenStream::from_str("\"hello world\"").unwrap(),
    //                             },
    //                         }),
    //                         syn::token::Semi {
    //                             spans: [Span::call_site()],
    //                         },
    //                     ),
    //                 ],
    //             }),
    //         }),
    //     ],
    // };

    let file = syn::File {
        shebang: None,
        attrs: vec![],
        items: vec![
            syn::Item::Mod(syn::ItemMod {
                attrs: vec![],
                vis: vis_inherited(),
                mod_token: mod_token(),
                ident: ident("fe_prelude"),
                content: None,
                semi: Some(semi_token()),
            }),
            syn::Item::Use(syn::ItemUse {
                attrs: vec![],
                vis: vis_inherited(),
                use_token: use_token(),
                leading_colon: None,
                semi_token: semi_token(),
                tree: syn::UseTree::Path(syn::UsePath {
                    ident: ident("fe_prelude"),
                    colon2_token: colon2_token(),
                    tree: Box::new(syn::UseTree::Glob(syn::UseGlob {
                        star_token: star_token(),
                    })),
                }),
            }),
            syn::Item::Fn(syn::ItemFn {
                attrs: vec![],
                vis: vis_public(),
                sig: syn::Signature {
                    constness: None,
                    asyncness: None,
                    unsafety: None,
                    abi: None,
                    fn_token: fn_token(),
                    ident: ident("main"),
                    generics: generics(vec![]),
                    paren_token: paren_token(),
                    inputs: fn_inputs(vec![]),
                    variadic: None,
                    output: return_type_void(),
                },
                block: Box::new(syn::Block {
                    brace_token: brace_token(),
                    stmts: vec![
                        syn::Stmt::Semi(
                            syn::Expr::Call(
                                syn::ExprCall {
                                    attrs: vec![],
                                    func: Box::new(syn::Expr::Path(syn::ExprPath {
                                        attrs: vec![],
                                        qself: None,
                                        path: syn::Path {
                                            leading_colon: None,
                                            segments: punctuated(vec![path_seg("print")], || colon2_token()),
                                        },
                                    })),
                                    args: punctuated(vec![
                                        syn::Expr::Lit(syn::ExprLit {
                                            attrs: vec![],
                                            lit: lit_str("hello world"),
                                        }),
                                        syn::Expr::Lit(syn::ExprLit {
                                            attrs: vec![],
                                            lit: lit_bool(true),
                                        }),
                                    ], || comma_token()),
                                    paren_token: paren_token(),
                                },
                            ),
                            semi_token(),
                        )
                    ],
                }),
            }),
        ],
    };

    return Ok(file.into_token_stream().to_string());
}

fn span() -> Span {
    return Span::call_site();
}

fn ident(name: &str) -> syn::Ident {
    return syn::Ident::new(name, span());
}

fn path_seg(name: &str) -> syn::PathSegment {
    return syn::PathSegment {
        ident: ident(name),
        arguments: syn::PathArguments::None,
    };
}

fn semi_token() -> syn::token::Semi {
    return syn::token::Semi {
        spans: [span()],
    };
}

fn pub_token() -> syn::token::Pub {
    return syn::token::Pub {
        span: span(),
    };
}

fn mod_token() -> syn::token::Mod {
    return syn::token::Mod {
        span: span(),
    };
}

fn use_token() -> syn::token::Use {
    return syn::token::Use {
        span: span(),
    };
}

fn fn_token() -> syn::token::Fn {
    return syn::token::Fn {
        span: span(),
    };
}

fn paren_token() -> syn::token::Paren {
    return syn::token::Paren {
        span: span(),
    };
}

fn colon2_token() -> syn::token::Colon2 {
    return syn::token::Colon2 {
        spans: [span(), span()],
    };
}

fn brace_token() -> syn::token::Brace {
    return syn::token::Brace {
        span: span(),
    };
}

fn star_token() -> syn::token::Star {
    return syn::token::Star {
        spans: [span()],
    };
}

fn lt_token() -> syn::token::Lt {
    return syn::token::Lt {
        spans: [span()],
    };
}

fn gt_token() -> syn::token::Gt {
    return syn::token::Gt {
        spans: [span()],
    };
}

fn comma_token() -> syn::token::Comma {
    return syn::token::Comma {
        spans: [span()],
    };
}

fn return_type_void() -> syn::ReturnType {
    return syn::ReturnType::Default;
}

fn vis_public() -> syn::Visibility {
    return syn::Visibility::Public(syn::VisPublic {
        pub_token: pub_token(),
    });
}

fn vis_inherited() -> syn::Visibility {
    return syn::Visibility::Inherited;
}

fn punctuated<T, P>(items: Vec<T>, delim: fn() -> P) -> syn::punctuated::Punctuated<T, P>
    where P: std::default::Default
{
    let mut p = syn::punctuated::Punctuated::new();

    let mut is_first = true;

    for item in items.into_iter() {
        if is_first {
            p.push(item);
            is_first = false;
        } else {
            p.push_punct(delim());
            p.push_value(item);
        }
    }

    return p;
}

fn generics(params: Vec<syn::GenericParam>) -> syn::Generics {
    if params.len() == 0 {
        return syn::Generics {
            lt_token: None,
            gt_token: None,
            params: punctuated(params, || comma_token()),
            where_clause: None,
        };
    } else {
        return syn::Generics {
            lt_token: Some(lt_token()),
            gt_token: Some(gt_token()),
            params: punctuated(params, || comma_token()),
            where_clause: None,
        };
    }
}

fn fn_inputs(params: Vec<syn::FnArg>) -> syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma> {
    return punctuated(params, || comma_token());
}

fn lit_str(value: &str) -> syn::Lit {
    return syn::Lit::Str(syn::LitStr::new(value, span()));
}

fn lit_bool(value: bool) -> syn::Lit {
    return syn::Lit::Bool(syn::LitBool::new(value, span()));
}

