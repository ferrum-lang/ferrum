use super::*;

use super::super::semantics::*;

use syn::punctuated::Punctuated;

pub fn convert_to_syn_ast(sem_ast: SemanticAST) -> syn::File {
    let mut file = syn::File {
        shebang: None,
        attrs: vec![],
        items: vec![],
    };

    for r#mod in sem_ast.mods.into_iter() {
        file.items.push(convert_mod(r#mod));
    }

    for r#use in sem_ast.uses.into_iter() {
        file.items.push(convert_use(r#use));
    }

    for static_const in sem_ast.static_consts.into_iter() {
        file.items.push(convert_static_const(static_const));
    }

    for item in sem_ast.items.into_iter() {
        file.items.push(convert_item(item));
    }

    return file;
}

fn convert_mod(r#mod: Mod) -> syn::Item {
    return syn::Item::Mod(syn::ItemMod {
        attrs: vec![],
        vis: vis(r#mod.is_public),
        mod_token: mod_token(),
        ident: ident(r#mod.name),
        content: None,
        semi: Some(semi_token()),
    });
}

fn convert_use(r#use: Use) -> syn::Item {
    return syn::Item::Use(syn::ItemUse {
        attrs: vec![],
        vis: vis(r#use.is_public),
        use_token: use_token(),
        leading_colon: None,
        semi_token: semi_token(),
        tree: convert_use_tree(r#use.tree),
    });
}

fn convert_use_tree(use_tree: UseTree) -> syn::UseTree {
    return match use_tree {
        UseTree::Path(path) => syn::UseTree::Path(syn::UsePath {
            colon2_token: colon2_token(),
            ident: ident(path.ident),
            tree: Box::new(convert_use_tree(*path.tree)),
        }),
        UseTree::Name(name) => syn::UseTree::Name(syn::UseName {
            ident: ident(name.name),
        }),
        UseTree::Group(group) => syn::UseTree::Group(syn::UseGroup {
            brace_token: brace_token(),
            items: group.items
                .into_iter()
                .map(|ut| convert_use_tree(ut))
                .collect(),
        }),
        UseTree::Glob => syn::UseTree::Glob(syn::UseGlob {
            star_token: star_token(),
        }),
    };
}

fn convert_static_const(static_const: StaticConst) -> syn::Item {
    return syn::Item::Const(syn::ItemConst {
        attrs: vec![],
        colon_token: colon_token(),
        const_token: const_token(),
        eq_token: eq_token(),
        semi_token: semi_token(),
        vis: vis(static_const.is_public),
        ident: ident(static_const.name),
        ty: Box::new(convert_type(*static_const.typ)),
        expr: Box::new(convert_expr(*static_const.expr)),
    });
}

fn convert_type(typ: Type) -> syn::Type {
    return match typ {
        Type::Array(arr) => syn::Type::Array(syn::TypeArray {
            bracket_token: bracket_token(),
            semi_token: semi_token(),
            elem: Box::new(convert_type(*arr.elem)),
            len: convert_expr(arr.len),
        }),
        Type::BareFn(bare_fn) => syn::Type::BareFn(syn::TypeBareFn {
            lifetimes: None,
            unsafety: None,
            abi: None,
            variadic: None,
            fn_token: fn_token(),
            paren_token: paren_token(),
            inputs: convert_bare_fn_params(bare_fn.inputs),
            output: convert_return_type(bare_fn.output),
        }),
        Type::ImplTrait(impl_trait) => syn::Type::ImplTrait(syn::TypeImplTrait {
            impl_token: impl_token(),
            bounds: convert_type_param_bounds(impl_trait.bounds),
        }),
        Type::Infer => syn::Type::Infer(syn::TypeInfer {
            underscore_token: underscore_token(),
        }),
        Type::Path(path) => syn::Type::Path(syn::TypePath {
            qself: None,
            path: convert_path(path.path),
        }),
        Type::Reference(reference) => syn::Type::Reference(syn::TypeReference {
            and_token: and_token(),
            lifetime: reference.lifetime.map(convert_lifetime),
            mutability: mutability(reference.is_mut),
            elem: Box::new(convert_type(*reference.elem)),
        }),
        Type::Slice(slice) => syn::Type::Slice(syn::TypeSlice {
            bracket_token: bracket_token(),
            elem: Box::new(convert_type(*slice.elem)),
        }),
        Type::TraitObject(trait_obj) => syn::Type::TraitObject(syn::TypeTraitObject {
            dyn_token: is_dyn(trait_obj.is_dyn),
            bounds: convert_type_param_bounds(trait_obj.bounds),
        }),
        Type::Tuple(tuple) => syn::Type::Tuple(syn::TypeTuple {
            paren_token: paren_token(),
            elems: punctuated(
                tuple.elems
                    .into_iter()
                    .map(convert_type)
                    .collect(),
                || comma_token()
            ),
        }),
    };
}

fn convert_bare_fn_params(params: Vec<BareFnParam>) -> Punctuated<syn::BareFnArg, syn::token::Comma> {
    return punctuated(
        params
            .into_iter()
            .map(|param| syn::BareFnArg {
                attrs: vec![],
                name: param.name.map(|name| (ident(name), colon_token())),
                ty: convert_type(param.typ),
            })
            .collect()
        ,
        || comma_token()
    );
}

fn convert_type_param_bounds(type_param_bounds: Vec<TypeParamBound>) -> Punctuated<syn::TypeParamBound, syn::token::Add> {
    return punctuated(
        type_param_bounds
            .into_iter()
            .map(|type_param_bound| convert_type_param_bound(type_param_bound))
            .collect(),
        || add_token()
    );
}

fn convert_type_param_bound(type_param_bound: TypeParamBound) -> syn::TypeParamBound {
    match type_param_bound {
        TypeParamBound::Trait(trait_bound) => syn::TypeParamBound::Trait(syn::TraitBound {
            paren_token: None,
            modifier: syn::TraitBoundModifier::None,
            lifetimes: None,
            path: convert_path(trait_bound.path),
        }),
        TypeParamBound::Lifetime(lifetime) => syn::TypeParamBound::Lifetime(convert_lifetime(lifetime)),
    }
}

fn convert_path(path: Path) -> syn::Path {
    return syn::Path {
        leading_colon: None,
        segments: punctuated(
            path.segments
                .into_iter()
                .map(convert_path_segment)
                .collect(),
            || colon2_token()
        ),
    };
}

fn convert_path_segment(segment: PathSegment) -> syn::PathSegment {
    return syn::PathSegment {
        arguments: match segment.arguments {
            PathArguments::None => syn::PathArguments::None,
            PathArguments::AngleBracketed(args) => syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                colon2_token: None,
                lt_token: lt_token(),
                gt_token: gt_token(),
                args: punctuated(
                    args.args
                        .into_iter()
                        .map(convert_generic_argument)
                        .collect(),
                    || comma_token()
                ),
            }),
            PathArguments::Parenthesized(args) => syn::PathArguments::Parenthesized(syn::ParenthesizedGenericArguments {
                paren_token: paren_token(),
                inputs: punctuated(
                    args.inputs
                        .into_iter()
                        .map(convert_type)
                        .collect(),
                    || comma_token()
                ),
                output: convert_return_type(args.output),
            }),
        },
        ident: ident(segment.ident),
    };
}

fn convert_generic_argument(generic_argument: GenericArgument) -> syn::GenericArgument {
    return match generic_argument {
        GenericArgument::Lifetime(lifetime) => syn::GenericArgument::Lifetime(convert_lifetime(lifetime)),
        GenericArgument::Type(typ) => syn::GenericArgument::Type(convert_type(typ)),
        GenericArgument::Binding(binding) => syn::GenericArgument::Binding(convert_binding(binding)),
        GenericArgument::Constraint(constraint) => syn::GenericArgument::Constraint(convert_constraint(constraint)),
        GenericArgument::Const(expr) => syn::GenericArgument::Const(convert_expr(expr)),
    };
}

fn convert_lifetime(lifetime: Lifetime) -> syn::Lifetime {
    return syn::Lifetime {
        apostrophe: span(),
        ident: ident(lifetime.name),
    };
}

fn convert_binding(binding: Binding) -> syn::Binding {
    return syn::Binding {
        ident: ident(binding.name),
        eq_token: eq_token(),
        ty: convert_type(binding.typ),
    };
}

fn convert_constraint(constraint: Constraint) -> syn::Constraint {
    return syn::Constraint {
        ident: ident(constraint.name),
        colon_token: colon_token(),
        bounds: convert_type_param_bounds(constraint.bounds),
    };
}

fn convert_return_type(return_type: ReturnType) -> syn::ReturnType {
    return match return_type {
        ReturnType::Type(typ) => syn::ReturnType::Type(
            r_arrow_token(),
            Box::new(convert_type(*typ))
        ),
        ReturnType::Default => syn::ReturnType::Default,
    };
}

fn convert_expr(expr: Expr) -> syn::Expr {
    return match expr {
        Expr::Array(arr) => syn::Expr::Array(syn::ExprArray {
            attrs: vec![],
            bracket_token: bracket_token(),
            elems: punctuated(
                arr.elems
                    .into_iter()
                    .map(convert_expr)
                    .collect(),
                || comma_token()
            ),
        }),
        Expr::Assign(assign) => syn::Expr::Assign(syn::ExprAssign {
            attrs: vec![],
            eq_token: eq_token(),
            left: Box::new(convert_expr(*assign.left)),
            right: Box::new(convert_expr(*assign.right)),
        }),
        Expr::AssignOp(assign_op) => syn::Expr::AssignOp(syn::ExprAssignOp {
            attrs: vec![],
            op: convert_bin_op(assign_op.op),
            left: Box::new(convert_expr(*assign_op.left)),
            right: Box::new(convert_expr(*assign_op.right)),
        }),
        Expr::Async(async_expr) => syn::Expr::Async(syn::ExprAsync {
            attrs: vec![],
            async_token: async_token(),
            capture: capture(async_expr.is_move),
            block: convert_block(async_expr.block),
        }),
        Expr::Await(await_expr) => syn::Expr::Await(syn::ExprAwait {
            attrs: vec![],
            await_token: await_token(),
            dot_token: dot_token(),
            base: Box::new(convert_expr(*await_expr.base)),
        }),
        Expr::Binary(bin) => syn::Expr::Binary(syn::ExprBinary {
            attrs: vec![],
            op: convert_bin_op(bin.op),
            left: Box::new(convert_expr(*bin.left)),
            right: Box::new(convert_expr(*bin.right)),
        }),
        Expr::Block(block) => syn::Expr::Block(syn::ExprBlock {
            attrs: vec![],
            label: None,
            block: convert_block(block),
        }),
        Expr::Break => syn::Expr::Break(syn::ExprBreak {
            attrs: vec![],
            break_token: break_token(),
            label: None,
            expr: None,
        }),
        Expr::Call(call) => syn::Expr::Call(syn::ExprCall {
            attrs: vec![],
            paren_token: paren_token(),
            func: Box::new(convert_expr(*call.func)),
            args: punctuated(
                call.args
                    .into_iter()
                    .map(convert_expr)
                    .collect(),
                || comma_token()
            ),
        }),
        Expr::Cast(cast) => syn::Expr::Cast(syn::ExprCast {
            attrs: vec![],
            as_token: as_token(),
            expr: Box::new(convert_expr(*cast.expr)),
            ty: Box::new(convert_type(*cast.typ)),
        }),
        Expr::Closure(closure) => syn::Expr::Closure(syn::ExprClosure {
            attrs: vec![],
            asyncness: asyncness(closure.is_async),
            capture: capture(closure.is_move),
            movability: None,
            or1_token: or_token(),
            or2_token: or_token(),
            inputs: punctuated(
                closure.params
                    .into_iter()
                    .map(convert_pattern)
                    .collect(),
                || comma_token()
            ),
            output: convert_return_type(closure.return_type),
            body: Box::new(convert_expr(*closure.body)),
        }),
        Expr::Continue => syn::Expr::Continue(syn::ExprContinue {
            attrs: vec![],
            label: None,
            continue_token: continue_token(),
        }),
        Expr::Field(field) => syn::Expr::Field(syn::ExprField {
            attrs: vec![],
            dot_token: dot_token(),
            base: Box::new(convert_expr(*field.base)),
            member: convert_member(field.member),
        }),
        Expr::ForLoop(for_loop) => syn::Expr::ForLoop(syn::ExprForLoop {
            attrs: vec![],
            for_token: for_token(),
            in_token: in_token(),
            label: None,
            pat: convert_pattern(for_loop.pattern),
            expr: Box::new(convert_expr(*for_loop.expr)),
            body: convert_block(for_loop.body),
        }),
        Expr::If(if_expr) => syn::Expr::If(syn::ExprIf {
            attrs: vec![],
            if_token: if_token(),
            cond: Box::new(convert_expr(*if_expr.condition)),
            then_branch: convert_block(if_expr.then_branch),
            else_branch: if_expr.else_branch.map(
                |expr| (else_token(), Box::new(convert_expr(*expr)))
            ),
        }),
        Expr::Index(index) => syn::Expr::Index(syn::ExprIndex {
            attrs: vec![],
            bracket_token: bracket_token(),
            expr: Box::new(convert_expr(*index.expr)),
            index: Box::new(convert_expr(*index.index)),
        }),
        Expr::Let(let_expr) => syn::Expr::Let(syn::ExprLet {
            attrs: vec![],
            let_token: let_token(),
            eq_token: eq_token(),
            pat: convert_pattern(let_expr.pattern),
            expr: Box::new(convert_expr(*let_expr.expr)),
        }),
        Expr::Lit(ExprLit { literal }) => syn::Expr::Lit(syn::ExprLit {
            attrs: vec![],
            lit: convert_lit(literal),
        }),
        Expr::Loop(loop_expr) => syn::Expr::Loop(syn::ExprLoop {
            attrs: vec![],
            label: None,
            loop_token: loop_token(),
            body: convert_block(loop_expr.body),
        }),
        Expr::Macro(ExprMacro { mac }) => syn::Expr::Macro(syn::ExprMacro {
            attrs: vec![],
            mac: convert_macro(mac),
        }),
        Expr::Match(match_expr) => syn::Expr::Match(syn::ExprMatch {
            attrs: vec![],
            match_token: match_token(),
            brace_token: brace_token(),
            expr: Box::new(convert_expr(*match_expr.expr)),
            arms: convert_match_arms(match_expr.arms),
        }),
        Expr::MethodCall(method_call) => syn::Expr::MethodCall(syn::ExprMethodCall {
            attrs: vec![],
            dot_token: dot_token(),
            paren_token: paren_token(),
            args: punctuated(
                method_call.args
                    .into_iter()
                    .map(|arg| convert_expr(arg))
                    .collect(),
                || comma_token(),
            ),
            receiver: Box::new(convert_expr(*method_call.receiver)),
            method: ident(method_call.method),
            turbofish: method_call.turbofish.map(|tf| syn::MethodTurbofish {
                colon2_token: colon2_token(),
                lt_token: lt_token(),
                gt_token: gt_token(),
                args: punctuated(
                    tf.args
                        .into_iter()
                        .map(convert_generic_method_argument)
                        .collect(),
                    || comma_token(),
                ),
            }),
        }),
        Expr::Paren(paren) => syn::Expr::Paren(syn::ExprParen {
            attrs: vec![],
            paren_token: paren_token(),
            expr: Box::new(convert_expr(*paren.expr)),
        }),
        Expr::Path(path) => syn::Expr::Path(syn::ExprPath {
            attrs: vec![],
            qself: None,
            path: convert_path(path.path),
        }),
        Expr::Range(range) => syn::Expr::Range(syn::ExprRange {
            attrs: vec![],
            from: range.from.map(|expr| Box::new(convert_expr(*expr))),
            to: range.to.map(|expr| Box::new(convert_expr(*expr))),
            limits: range_limits(range.to_inclusive),
        }),
        // Expr::Reference(reference) => syn::Expr::Reference(syn::ExprReference {
        //     attrs: vec![],
        //     and_token: and_token(),
        //     mutability: mutability(reference.is_mutable),
        //     expr: Box::new(convert_expr(*reference.expr)),
        //     raw: 
        // }),
        Expr::Reference(reference) => {
            #[allow(unused)]
            let expr = convert_expr(*reference.expr);
            
            if reference.is_mutable {
                syn::parse_quote!(&mut expr)
            } else {
                syn::parse_quote!(&expr)
            }
        },
        Expr::Repeat(repeat) => syn::Expr::Repeat(syn::ExprRepeat {
            attrs: vec![],
            bracket_token: bracket_token(),
            semi_token: semi_token(),
            expr: Box::new(convert_expr(*repeat.expr)),
            len: Box::new(convert_expr(*repeat.len)),
        }),
        Expr::Return(ret) => syn::Expr::Return(syn::ExprReturn {
            attrs: vec![],
            return_token: return_token(),
            expr: ret.expr.map(|expr| Box::new(convert_expr(*expr))),
        }),
        Expr::Struct(struct_expr) => syn::Expr::Struct(syn::ExprStruct {
            attrs: vec![],
            brace_token: brace_token(),
            path: convert_path(struct_expr.path),
            fields: punctuated(
                struct_expr.fields
                    .into_iter()
                    .map(convert_field_value)
                    .collect(),
                || comma_token(),
            ),
            dot2_token: struct_expr.rest.clone().map(|_| dot2_token()),
            rest: struct_expr.rest.map(|expr| Box::new(convert_expr(*expr))),
        }),
        Expr::Try(try_expr) => syn::Expr::Try(syn::ExprTry {
            attrs: vec![],
            question_token: question_token(),
            expr: Box::new(convert_expr(*try_expr.expr)),
        }),
        Expr::Tuple(tuple) => syn::Expr::Tuple(syn::ExprTuple {
            attrs: vec![],
            paren_token: paren_token(),
            elems: punctuated(
                tuple.elems
                    .into_iter()
                    .map(convert_expr)
                    .collect(),
                || comma_token()
            ),
        }),
        Expr::Type(typ) => syn::Expr::Type(syn::ExprType {
            attrs: vec![],
            colon_token: colon_token(),
            expr: Box::new(convert_expr(*typ.expr)),
            ty: Box::new(convert_type(*typ.typ)),
        }),
        Expr::Unary(unary) => syn::Expr::Unary(syn::ExprUnary {
            attrs: vec![],
            op: convert_un_op(unary.op),
            expr: Box::new(convert_expr(*unary.expr)),
        }),
        Expr::While(whil) => syn::Expr::While(syn::ExprWhile {
            attrs: vec![],
            label: None,
            while_token: while_token(),
            cond: Box::new(convert_expr(*whil.condition)),
            body: convert_block(whil.body),
        }),
    };
}

fn convert_generic_method_argument(arg: GenericMethodArgument) -> syn::GenericMethodArgument {
    return match arg {
        GenericMethodArgument::Const(expr) => syn::GenericMethodArgument::Const(convert_expr(expr)),
        GenericMethodArgument::Type(typ) => syn::GenericMethodArgument::Type(convert_type(typ)),
    };
}

fn convert_field_value(field_value: FieldValue) -> syn::FieldValue {
    let name = match &field_value.member {
        Member::Named(name) => name.clone(),
        Member::Indexed(index) => index.to_string(),
    };

    return syn::FieldValue {
        attrs: vec![],
        member: convert_member(field_value.member),
        colon_token: field_value.expr.clone().map(|_| colon_token()),
        expr: field_value.expr
            .map(convert_expr)
            .unwrap_or_else(|| syn::Expr::Path(syn::ExprPath {
                attrs: vec![],
                qself: None,
                path: syn::Path {
                    leading_colon: None,
                    segments: punctuated(
                        vec![syn::PathSegment {
                            ident: ident(name),
                            arguments: syn::PathArguments::None,
                        }],
                        || colon2_token()
                    ),
                },
            })),
    };
}

fn convert_un_op(un_op: UnaryOp) -> syn::UnOp {
    return match un_op {
        UnaryOp::Deref => syn::UnOp::Deref(star_token()),
        UnaryOp::Not => syn::UnOp::Not(bang_token()),
        UnaryOp::Neg => syn::UnOp::Neg(sub_token()),
    };
}

fn convert_bin_op(bin_op: BinOp) -> syn::BinOp {
    return match bin_op {
        BinOp::Add => syn::BinOp::Add(add_token()),
        BinOp::Sub => syn::BinOp::Sub(sub_token()),
        BinOp::Mul => syn::BinOp::Mul(star_token()),
        BinOp::Div => syn::BinOp::Div(div_token()),
        BinOp::Rem => syn::BinOp::Rem(rem_token()),
        BinOp::And => syn::BinOp::And(and_and_token()),
        BinOp::Or => syn::BinOp::Or(or_or_token()),
        BinOp::Eq => syn::BinOp::Eq(eq_eq_token()),
        BinOp::Lt => syn::BinOp::Lt(lt_token()),
        BinOp::Le => syn::BinOp::Le(le_token()),
        BinOp::Ne => syn::BinOp::Ne(ne_token()),
        BinOp::Ge => syn::BinOp::Ge(ge_token()),
        BinOp::Gt => syn::BinOp::Gt(gt_token()),
        BinOp::AddEq => syn::BinOp::AddEq(add_eq_token()),
        BinOp::SubEq => syn::BinOp::SubEq(sub_eq_token()),
        BinOp::MulEq => syn::BinOp::MulEq(mul_eq_token()),
        BinOp::DivEq => syn::BinOp::DivEq(div_eq_token()),
        BinOp::RemEq => syn::BinOp::RemEq(rem_eq_token()),
    };
}

fn convert_pattern(pattern: Pattern) -> syn::Pat {
    return match pattern {
        Pattern::Ident(name) => syn::Pat::Ident(syn::PatIdent {
            attrs: vec![],
            by_ref: by_ref(name.is_ref),
            mutability: mutability(name.is_mut),
            ident: ident(name.name),
            subpat: None,
        }),
        Pattern::Lit(lit) => syn::Pat::Lit(syn::PatLit {
            attrs: vec![],
            expr: Box::new(convert_expr(*lit.expr)),
        }),
        Pattern::Or(or) => syn::Pat::Or(syn::PatOr {
            attrs: vec![],
            leading_vert: None,
            cases: punctuated(
                or.cases
                    .into_iter()
                    .map(convert_pattern)
                    .collect(),
                || or_token()
            ),
        }),
        Pattern::Path(path) => syn::Pat::Path(syn::PatPath {
            attrs: vec![],
            qself: None,
            path: convert_path(path.path),
        }),
        Pattern::Range(range) => syn::Pat::Range(syn::PatRange {
            attrs: vec![],
            lo: Box::new(convert_expr(*range.low)),
            hi: Box::new(convert_expr(*range.high)),
            limits: range_limits(range.inclusive_high),
        }),
        Pattern::Reference(reference) => syn::Pat::Reference(syn::PatReference {
            attrs: vec![],
            and_token: and_token(),
            mutability: mutability(reference.is_mutable),
            pat: Box::new(convert_pattern(*reference.pattern)),
        }),
        Pattern::Rest => syn::Pat::Rest(syn::PatRest {
            attrs: vec![],
            dot2_token: dot2_token(),
        }),
        Pattern::Slice(slice) => syn::Pat::Slice(syn::PatSlice {
            attrs: vec![],
            bracket_token: bracket_token(),
            elems: punctuated(
                slice.elems
                    .into_iter()
                    .map(convert_pattern)
                    .collect(),
                || comma_token()
            ),
        }),
        Pattern::Struct(struct_pat) => syn::Pat::Struct(syn::PatStruct {
            attrs: vec![],
            brace_token: brace_token(),
            dot2_token: None,
            path: convert_path(struct_pat.path),
            fields: punctuated(
                struct_pat.fields
                    .into_iter()
                    .map(|field| syn::FieldPat {
                        attrs: vec![],
                        colon_token: field.pattern.clone().map(|_| colon_token()),
                        member: convert_member(field.member.clone()),
                        pat: field.pattern
                            .map(|pat| Box::new(convert_pattern(*pat)))
                            .unwrap_or_else(|| Box::new(syn::Pat::Ident(syn::PatIdent {
                                attrs: vec![],
                                by_ref: None,
                                mutability: None,
                                ident: ident(match field.member {
                                    Member::Named(name) => name.clone(),
                                    Member::Indexed(index) => index.to_string(),
                                }),
                                subpat: None,
                            }))),
                    })
                    .collect(),
                || comma_token()
            )
        }),
        Pattern::Tuple(tuple) => syn::Pat::Tuple(syn::PatTuple {
            attrs: vec![],
            paren_token: paren_token(),
            elems: punctuated(
                tuple.elems
                    .into_iter()
                    .map(convert_pattern)
                    .collect(),
                || comma_token()
            )
        }),
        Pattern::TupleStruct(tuple_struct) => syn::Pat::TupleStruct(syn::PatTupleStruct {
            attrs: vec![],
            path: convert_path(tuple_struct.path),
            pat: syn::PatTuple {
                attrs: vec![],
                paren_token: paren_token(),
                elems: punctuated(
                    tuple_struct.pat_tuple.elems
                        .into_iter()
                        .map(convert_pattern)
                        .collect(),
                    || comma_token()
                ),
            },
        }),
        Pattern::Type(typ) => syn::Pat::Type(syn::PatType {
            attrs: vec![],
            colon_token: colon_token(),
            ty: Box::new(convert_type(*typ.typ)),
            pat: Box::new(convert_pattern(*typ.pattern)),
        }),
    };
}

fn convert_member(member: Member) -> syn::Member {
    return match member {
        Member::Named(name) => syn::Member::Named(ident(name)),
        Member::Indexed(index) => syn::Member::Unnamed(syn::Index {
            index,
            span: span(),
        })
    };
}

fn convert_lit(literal: Literal) -> syn::Lit {
    return match literal {
        Literal::Str(val) => syn::Lit::Str(syn::LitStr::new(&val, span())),
        Literal::Char(val) => syn::Lit::Char(syn::LitChar::new(val, span())),
        Literal::Int(val) => syn::Lit::Int(syn::LitInt::new(&val, span())),
        Literal::Float(val) => syn::Lit::Float(syn::LitFloat::new(&val, span())),
        Literal::Bool(val) => syn::Lit::Bool(syn::LitBool::new(val, span())),
    };
}

fn convert_macro(mac: Macro) -> syn::Macro {
    return syn::Macro {
        bang_token: bang_token(),
        delimiter: match mac.delimiter {
            MacroDelimiter::Bracket => syn::MacroDelimiter::Bracket(bracket_token()),
            MacroDelimiter::Paren => syn::MacroDelimiter::Paren(paren_token()),
            MacroDelimiter::Brace => syn::MacroDelimiter::Brace(brace_token()),
        },
        path: convert_path(mac.path),
        tokens: token_stream(mac.values),
    };
}

fn convert_match_arms(arms: Vec<MatchArm>) -> Vec<syn::Arm> {
    return arms
        .into_iter()
        .map(|arm| syn::Arm {
            attrs: vec![],
            fat_arrow_token: fat_arrow_token(),
            comma: Some(comma_token()),
            pat: convert_pattern(arm.pattern),
            guard: arm.guard.map(|guard| (if_token(), Box::new(convert_expr(*guard)))),
            body: Box::new(convert_expr(*arm.body)),
        })
        .collect();
}

fn convert_block(block: Block) -> syn::Block {
    return syn::Block {
        brace_token: brace_token(),
        stmts: block.statements
            .into_iter()
            .map(convert_statement)
            .collect(),
    };
}

fn convert_statement(statement: Statement) -> syn::Stmt {
    return match statement {
        Statement::Local(local) => syn::Stmt::Local(convert_local(local)),
        Statement::Item(item) => syn::Stmt::Item(convert_item(item)),
        Statement::Expr(expr) => syn::Stmt::Expr(convert_expr(expr)),
        Statement::Semi(expr) => syn::Stmt::Semi(convert_expr(expr), semi_token()),
    };
}

fn convert_local(local: Local) -> syn::Local {
    return syn::Local {
        attrs: vec![],
        let_token: let_token(),
        pat: convert_pattern(local.pattern),
        init: local.init.map(|expr| (eq_token(), Box::new(convert_expr(*expr)))),
        semi_token: semi_token(),
    };
}

fn convert_item(item: Item) -> syn::Item {
    return match item {
        Item::Const(item) => syn::Item::Const(syn::ItemConst {
            attrs: vec![],
            colon_token: colon_token(),
            const_token: const_token(),
            eq_token: eq_token(),
            semi_token: semi_token(),
            vis: vis(item.is_public),
            ident: ident(item.name),
            ty: Box::new(convert_type(*item.typ)),
            expr: Box::new(convert_expr(*item.expr)),
        }),
        Item::Enum(item) => syn::Item::Enum(syn::ItemEnum {
            attrs: vec![],
            brace_token: brace_token(),
            enum_token: enum_token(),
            vis: vis(item.is_public),
            generics: convert_generics(item.generics),
            ident: ident(item.name),
            variants: punctuated(
                item.variants
                    .into_iter()
                    .map(|variant| syn::Variant {
                        attrs: vec![],
                        ident: ident(variant.name),
                        discriminant: None,
                        fields: convert_fields(variant.fields),
                    })
                    .collect(),
                || comma_token()
            ),
        }),
        Item::Fn(item) => syn::Item::Fn(syn::ItemFn {
            attrs: vec![],
            vis: vis(item.is_public),
            sig: syn::Signature {
                abi: None,
                unsafety: None,
                variadic: None,
                paren_token: paren_token(),
                asyncness: asyncness(item.signature.is_async),
                constness: constness(item.signature.is_const),
                fn_token: fn_token(),
                generics: convert_generics(item.signature.generics),
                ident: ident(item.signature.name),
                inputs: punctuated(
                    item.signature.params
                        .into_iter()
                        .map(|param| match param {
                            FnParam::Self_(self_) => syn::FnArg::Receiver(syn::Receiver {
                                attrs: vec![],
                                self_token: self_token(),
                                mutability: mutability(self_.is_mutable),
                                reference: self_.reference.map(
                                    |reference| (and_token(), reference.lifetime.map(convert_lifetime))
                                ),
                            }),
                            FnParam::Typed(typed) => syn::FnArg::Typed(syn::PatType {
                                attrs: vec![],
                                pat: Box::new(convert_pattern(*typed.pattern)),
                                colon_token: colon_token(),
                                ty: Box::new(convert_type(*typed.typ)),
                            }),
                        })
                        .collect(),
                    || comma_token()
                ),
                output: convert_return_type(item.signature.return_type),
            },
            block: Box::new(convert_block(*item.block)),
        }),
        Item::Impl(item) => syn::Item::Impl(syn::ItemImpl {
            attrs: vec![],
            unsafety: None,
            defaultness: None,
            brace_token: brace_token(),
            impl_token: impl_token(),
            generics: convert_generics(item.generics),
            trait_: item.trait_.map(|path| (None, convert_path(path), for_token())),
            self_ty: Box::new(convert_type(*item.self_type)),
            items: item.items
                .into_iter()
                .map(|item| match item {
                    ImplItem::Const(item) => syn::ImplItem::Const(syn::ImplItemConst {
                        attrs: vec![],
                        defaultness: None,
                        colon_token: colon_token(),
                        const_token: const_token(),
                        eq_token: eq_token(),
                        semi_token: semi_token(),
                        vis: vis(item.is_public),
                        ident: ident(item.name),
                        ty: convert_type(item.typ),
                        expr: convert_expr(item.expr),
                    }),
                    ImplItem::Type(item) => syn::ImplItem::Type(syn::ImplItemType {
                        attrs: vec![],
                        defaultness: None,
                        eq_token: eq_token(),
                        semi_token: semi_token(),
                        type_token: type_token(),
                        vis: vis(item.is_public),
                        generics: convert_generics(item.generics),
                        ident: ident(item.name),
                        ty: convert_type(item.typ),
                    }),
                    ImplItem::Method(item) => syn::ImplItem::Method(syn::ImplItemMethod {
                        attrs: vec![],
                        defaultness: None,
                        vis: vis(item.is_public),
                        sig: syn::Signature {
                            abi: None,
                            unsafety: None,
                            variadic: None,
                            paren_token: paren_token(),
                            asyncness: asyncness(item.signature.is_async),
                            constness: constness(item.signature.is_const),
                            fn_token: fn_token(),
                            generics: convert_generics(item.signature.generics),
                            ident: ident(item.signature.name),
                            inputs: punctuated(
                                item.signature.inputs
                                    .into_iter()
                                    .map(|param| match param {
                                        FnParam::Self_(self_) => syn::FnArg::Receiver(syn::Receiver {
                                            attrs: vec![],
                                            self_token: self_token(),
                                            mutability: mutability(self_.is_mutable),
                                            reference: self_.reference.map(
                                                |reference| (and_token(), reference.lifetime.map(convert_lifetime))
                                            ),
                                        }),
                                        FnParam::Typed(typed) => syn::FnArg::Typed(syn::PatType {
                                            attrs: vec![],
                                            pat: Box::new(convert_pattern(*typed.pattern)),
                                            colon_token: colon_token(),
                                            ty: Box::new(convert_type(*typed.typ)),
                                        }),
                                    })
                                    .collect(),
                                || comma_token()
                            ),
                            output: convert_return_type(item.signature.output),
                        },
                        block: convert_block(item.block),
                    }),
                })
            .collect(),
        }),
        Item::Struct(item) => syn::Item::Struct(syn::ItemStruct {
            attrs: vec![],
            struct_token: struct_token(),
            semi_token: Some(semi_token()),
            vis: vis(item.is_public),
            ident: ident(item.name),
            generics: convert_generics(item.generics),
            fields: convert_fields(item.fields),
        }),
        Item::Trait(item) => syn::Item::Trait(syn::ItemTrait {
            attrs: vec![],
            unsafety: None,
            auto_token: None,
            brace_token: brace_token(),
            trait_token: trait_token(),
            colon_token: if item.supertraits.is_empty() { None } else { Some(colon_token()) },
            supertraits: punctuated(
                item.supertraits
                    .into_iter()
                    .map(convert_type_param_bound)
                    .collect(),
                || add_token()
            ),
            vis: vis(item.is_public),
            ident: ident(item.name),
            generics: convert_generics(item.generics),
            items: item.items
                .into_iter()
                .map(|item| match item {
                    TraitItem::Const(item) => syn::TraitItem::Const(syn::TraitItemConst {
                        attrs: vec![],
                        colon_token: colon_token(),
                        const_token: const_token(),
                        semi_token: semi_token(),
                        ident: ident(item.name),
                        ty: convert_type(item.typ),
                        default: item.default.map(|expr| (eq_token(), convert_expr(expr))),
                    }),
                    TraitItem::Type(item) => syn::TraitItem::Type(syn::TraitItemType {
                        attrs: vec![],
                        type_token: type_token(),
                        semi_token: semi_token(),
                        colon_token: if item.bounds.is_empty() { None } else { Some(colon_token()) },
                        bounds: convert_type_param_bounds(item.bounds),
                        default: item.default.map(|typ| (eq_token(), convert_type(typ))),
                        generics: convert_generics(item.generics),
                        ident: ident(item.name),
                    }),
                    TraitItem::Method(item) => syn::TraitItem::Method(syn::TraitItemMethod {
                        attrs: vec![],
                        semi_token: Some(semi_token()),
                        sig: syn::Signature {
                            abi: None,
                            unsafety: None,
                            variadic: None,
                            paren_token: paren_token(),
                            asyncness: asyncness(item.signature.is_async),
                            constness: constness(item.signature.is_const),
                            fn_token: fn_token(),
                            generics: convert_generics(item.signature.generics),
                            ident: ident(item.signature.name),
                            inputs: punctuated(
                                item.signature.inputs
                                    .into_iter()
                                    .map(|param| match param {
                                        FnParam::Self_(self_) => syn::FnArg::Receiver(syn::Receiver {
                                            attrs: vec![],
                                            self_token: self_token(),
                                            mutability: mutability(self_.is_mutable),
                                            reference: self_.reference.map(
                                                |reference| (and_token(), reference.lifetime.map(convert_lifetime))
                                            ),
                                        }),
                                        FnParam::Typed(typed) => syn::FnArg::Typed(syn::PatType {
                                            attrs: vec![],
                                            pat: Box::new(convert_pattern(*typed.pattern)),
                                            colon_token: colon_token(),
                                            ty: Box::new(convert_type(*typed.typ)),
                                        }),
                                    })
                                    .collect(),
                                || comma_token()
                            ),
                            output: convert_return_type(item.signature.output),
                        },
                        default: item.default.map(convert_block),
                    }),
                })
            .collect(),
        }),
        Item::Type(item) => syn::Item::Type(syn::ItemType {
            attrs: vec![],
            eq_token: eq_token(),
            type_token: type_token(),
            semi_token: semi_token(),
            vis: vis(item.is_public),
            ident: ident(item.name),
            generics: convert_generics(item.generics),
            ty: Box::new(convert_type(*item.typ)),
        }),
    };
}

fn convert_fields(fields: Fields) -> syn::Fields {
    return match fields {
        Fields::Unit => syn::Fields::Unit,
        Fields::Named(field) => syn::Fields::Named(syn::FieldsNamed {
            brace_token: brace_token(),
            named: punctuated(
                field.fields
                    .into_iter()
                    .map(|field| syn::Field {
                        attrs: vec![],
                        vis: vis_public(),
                        ident: Some(ident(field.name)),
                        colon_token: Some(colon_token()),
                        ty: convert_type(field.typ),
                    })
                    .collect(),
                || comma_token()
            ),
        }),
        Fields::Unnamed(field) => syn::Fields::Unnamed(syn::FieldsUnnamed {
            paren_token: paren_token(),
            unnamed: punctuated(
                field.fields
                    .into_iter()
                    .map(|field| syn::Field {
                        attrs: vec![],
                        vis: vis_public(),
                        ident: None,
                        colon_token: None,
                        ty: convert_type(field.typ),
                    })
                    .collect(),
                || comma_token()
            ),
        }),
    };
}

fn convert_generics(generics: Generics) -> syn::Generics {
    let is_empty = generics.params.is_empty();

    return syn::Generics {
        lt_token: if is_empty { Some(lt_token()) } else { None },
        gt_token: if is_empty { Some(gt_token()) } else { None },
        params: punctuated(
            generics.params
                .into_iter()
                .map(|param| match param {
                    GenericParam::Type(param) => syn::GenericParam::Type(syn::TypeParam {
                        attrs: vec![],
                        eq_token: None,
                        default: None,
                        colon_token: if param.bounds.is_empty() { None } else { Some(colon_token()) },
                        bounds: convert_type_param_bounds(param.bounds),
                        ident: ident(param.ident),
                    }),
                    GenericParam::Lifetime(param) => syn::GenericParam::Lifetime(syn::LifetimeDef {
                        attrs: vec![],
                        lifetime: convert_lifetime(param.lifetime),
                        colon_token: if param.bounds.is_empty() { None } else { Some(colon_token()) },
                        bounds: punctuated(
                            param.bounds
                                .into_iter()
                                .map(convert_lifetime)
                                .collect(),
                            || add_token()
                        ),
                    }),
                    GenericParam::Const(param) => syn::GenericParam::Const(syn::ConstParam {
                        attrs: vec![],
                        const_token: const_token(),
                        colon_token: colon_token(),
                        ident: ident(param.ident),
                        ty: convert_type(param.typ),
                        eq_token: param.default.clone().map(|_| eq_token()),
                        default: param.default.map(convert_expr),
                    }),
                })
                .collect(),
            || comma_token()
        ),
        where_clause: None,
    };
}

fn ident(name: String) -> syn::Ident {
    return syn::Ident::new(&name, span());
}

fn vis(is_public: bool) -> syn::Visibility {
    if is_public {
        return vis_public();
    } else {
        return vis_inherited();
    }
}

fn vis_public() -> syn::Visibility {
    return syn::Visibility::Public(syn::VisPublic {
        pub_token: pub_token(),
    });
}

fn vis_inherited() -> syn::Visibility {
    return syn::Visibility::Inherited;
}

fn mutability(is_mut: bool) -> Option<syn::token::Mut> {
    if is_mut {
        return Some(mut_token());
    } else {
        return None;
    }
}

fn by_ref(is_ref: bool) -> Option<syn::token::Ref> {
    if is_ref {
        return Some(ref_token());
    } else {
        return None;
    }
}

fn is_dyn(is_dyn: bool) -> Option<syn::token::Dyn> {
    if is_dyn {
        return Some(dyn_token());
    } else {
        return None;
    }
}

fn capture(is_move: bool) -> Option<syn::token::Move> {
    if is_move {
        return Some(move_token());
    } else {
        return None;
    }
}

fn asyncness(is_async: bool) -> Option<syn::token::Async> {
    if is_async {
        return Some(async_token());
    } else {
        return None;
    }
}

fn constness(is_const: bool) -> Option<syn::token::Const> {
    if is_const {
        return Some(const_token());
    } else {
        return None;
    }
}

fn range_limits(is_inclusive: bool) -> syn::RangeLimits {
    if is_inclusive {
        return syn::RangeLimits::Closed(dot_dot_eq_token());
    } else {
        return syn::RangeLimits::HalfOpen(dot2_token());
    }
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

fn token_stream(values: Vec<String>) -> proc_macro2::TokenStream {
    todo!();
}

