//! Implementation of the `#[activity]` and `#[workflow]` macros.
//! If we implemented these in `hoover_macro` crate, a proc_macro crate, we couldn't unit test their internals.
//! So we have to implement them in a separate crate that's not a proc_macro crate.
pub use syn;

use proc_macro2::TokenStream;
use std::boxed::Box;

use quote::quote;
use syn::GenericArgument;
use syn::PathArguments;
use syn::ReturnType;
use syn::Type;
use syn::{FnArg, ItemFn};

/// Get activity function argument type `T`
/// from a function declared like this:
/// `#[activity] fn foo(x: T) -> anyhow::Result<V> { ... }`
fn extract_activity_arg_type(f: &ItemFn) -> Box<syn::Type> {
    let args = f.sig.inputs.clone();
    let args_list = args.iter().collect::<Vec<_>>();
    assert!(
        args_list.len() == 1,
        "#[activity] must have exactly one argument"
    );
    let arg = args_list[0];
    let FnArg::Typed(arg_pattern) = arg else {
        panic!("activity argument must be a type, not self");
    };

    arg_pattern.ty.clone()
}

/// Get activity function return type `V`
/// from a function declared like this:
/// `#[activity] fn foo(x: T) -> anyhow::Result<V> { ... }`
fn extract_activity_return_type(f: &ItemFn) -> Box<syn::Type> {
    let ReturnType::Type(_, ret) = f.sig.output.clone() else {
        panic!("activity return type must be anyhow::Result<T>, not empty");
    };
    let Type::Path(ret) = *ret else {
        panic!("invalid return type")
    };
    let last_segment = ret.path.segments.last().expect("return type has segment");
    assert!(last_segment.ident == "Result", "not a result");
    let PathArguments::AngleBracketed(args) = last_segment.arguments.clone() else {
        panic!("invalid return type")
    };
    assert!(
        args.args.len() == 1,
        "invalid activity return type: plz use anyhow::Result<T>"
    );
    let GenericArgument::Type(ret) = args.args[0].clone() else {
        panic!("invalid return type")
    };
    Box::new(ret)
}

/// Macro extracts argument and return type from function definition,
/// then generates the `make_activity/_sync` macro call.
///
/// Starting from this:
/// `#[activity] async fn foo(x: T) -> anyhow::Result<V> { ... }`
/// it generates this:
/// `::hoover3_taskdef::make_activity!(foo, T, V); async fn foo(x: T) -> Result<V> { ... }`
pub fn activity(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let queue_name = syn::parse2::<syn::Expr>(attrs).expect("parse activity queue name");
    let f = syn::parse2::<syn::ItemFn>(item).expect("parse activity function");

    let macro_arg_type = extract_activity_arg_type(&f);
    let macro_ret_type = extract_activity_return_type(&f);

    let f_args = &f.sig.inputs;
    let f_out = &f.sig.output;
    let f_vis = &f.vis;
    let f_async = &f.sig.asyncness;
    let f_name = &f.sig.ident;
    let f_body = &f.block;

    let macro_name = if f_async.is_some() {
        "make_activity"
    } else {
        "make_activity_sync"
    };
    let macro_name = format!("::hoover3_taskdef::{}", macro_name);
    let macro_name: syn::Path = syn::parse_str(&macro_name).expect("parse macro name");

    let result = quote! {
        #macro_name!(#queue_name, #f_name, #macro_arg_type, #macro_ret_type);
        #f_vis #f_async fn #f_name(#f_args) #f_out #f_body
    };

    result
}

/// Get workflow function argument type `T`
/// from a function declared like this:
/// `#[workflow] async fn foo(ctx: WfContext, x: T) -> anyhow::Result<V> { ... }`
fn extract_workflow_arg_type(f: &ItemFn) -> Box<syn::Type> {
    let args = f.sig.inputs.clone();
    let args_list = args.iter().collect::<Vec<_>>();
    assert!(
        args_list.len() == 2,
        "#[workflow] must have exactly two arguments (ctx and input)"
    );
    let arg = args_list[1];
    let FnArg::Typed(arg_pattern) = arg else {
        panic!("workflow argument must be a type, not self");
    };

    arg_pattern.ty.clone()
}

/// Get workflow function return type `V`
/// from a function declared like this:
/// `#[workflow] async fn foo(ctx: WfContext, x: T) -> WorkflowResult<V> { ... }`
fn extract_workflow_return_type(f: &ItemFn) -> Box<syn::Type> {
    let ReturnType::Type(_, ret) = f.sig.output.clone() else {
        panic!("workflow return type must be WorkflowResult<T>, not empty");
    };
    let Type::Path(ret) = *ret else {
        panic!("invalid return type")
    };
    let last_segment = ret
        .path
        .segments
        .last()
        .expect("workflow return type has segment");
    assert!(
        last_segment.ident == "WorkflowResult",
        "not a WorkflowResult"
    );
    let PathArguments::AngleBracketed(args) = last_segment.arguments.clone() else {
        panic!("invalid return type")
    };
    assert!(
        args.args.len() == 1,
        "invalid workflow return type: plz use WorkflowResult<T>"
    );
    let GenericArgument::Type(ret) = args.args[0].clone() else {
        panic!("invalid return type")
    };
    Box::new(ret)
}

/// Macro extracts argument and return type from function definition,
/// then generates the `make_workflow` macro call.
///
/// Starting from this:
/// `#[workflow] async fn foo(ctx: WfContext, x: T) -> WorkflowResult<V> { ... }`
/// it generates this:
/// `::hoover3_taskdef::make_workflow!(foo, T, V); async fn foo(ctx: WfContext, x: T) -> WorkflowResult<V> { ... }`
pub fn workflow(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let queue_name = syn::parse2::<syn::Expr>(attrs).expect("parse workflow queue name");
    let f = syn::parse2::<syn::ItemFn>(item).expect("parse workflow function");

    let macro_arg_type = extract_workflow_arg_type(&f);
    let macro_ret_type = extract_workflow_return_type(&f);

    let f_args = &f.sig.inputs;
    let f_out = &f.sig.output;
    let f_vis = &f.vis;
    let f_async = &f.sig.asyncness;
    assert!(f_async.is_some(), "workflow must be async");
    let f_name = &f.sig.ident;
    let f_body = &f.block;

    let macro_name = "::hoover3_taskdef::make_workflow";
    let macro_name: syn::Path = syn::parse_str(macro_name).expect("parse workflow macro name");

    let result = quote! {
        #macro_name!(#queue_name, #f_name, #macro_arg_type, #macro_ret_type);
        #f_vis #f_async fn #f_name(#f_args) #f_out #f_body
    };

    result
}

#[test]
fn test_activity_sync() {
    let item = quote! {
        fn foo(x: u64) -> Result<u64> {
            x + 1
        }
    };
    let args = quote! { "task_queue" };
    let act = activity(args, item);
    assert_eq!(format!("{}", act), ":: hoover3_taskdef :: make_activity_sync ! (\"task_queue\" , foo , u64 , u64) ; fn foo (x : u64) -> Result < u64 > { x + 1 }");
}

#[test]
fn test_activity_async() {
    let item = quote! {
        pub async fn foo(x: u64) -> Result<u64> {
            x + 1
        }
    };
    let args = quote! { "task_queue" };
    let act = activity(args, item);
    assert_eq!(format!("{}", act), ":: hoover3_taskdef :: make_activity ! (\"task_queue\" , foo , u64 , u64) ; pub async fn foo (x : u64) -> Result < u64 > { x + 1 }");
}

#[test]
fn test_activity_async_with_tuple() {
    let item = quote! {
        pub async fn foo((x, y): (u64, u64)) -> Result<u64> {
            x + 1
        }
    };
    let args = quote! { "task_queue" };
    let act = activity(args, item);
    assert_eq!(format!("{}", act), ":: hoover3_taskdef :: make_activity ! (\"task_queue\" , foo , (u64 , u64) , u64) ; pub async fn foo ((x , y) : (u64 , u64)) -> Result < u64 > { x + 1 }");
}

#[test]
fn test_workflow() {
    let item = quote! {
        async fn foo(ctx: WfContext, x: u64) -> WorkflowResult<u64> {
            Ok(WfExitValue::Normal(x + 1))
        }
    };
    let args = quote! { "task_queue" };
    let wf = workflow(args, item);
    assert_eq!(format!("{}", wf), ":: hoover3_taskdef :: make_workflow ! (\"task_queue\" , foo , u64 , u64) ; async fn foo (ctx : WfContext , x : u64) -> WorkflowResult < u64 > { Ok (WfExitValue :: Normal (x + 1)) }");
}
