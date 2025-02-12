#![allow(dead_code)]
#![allow(unused_imports, unused_mut, unused_variables)]

use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::{format_ident};


use syn::{parse_macro_input, Data, DeriveInput, Variant};

#[proc_macro]
pub fn use_generator(tokens: TokenStream) -> TokenStream {
    //tokens

    let ast: syn::Expr = syn::parse(tokens).unwrap();
    let mut generator_function: TokenStream = "let generator_function = |usize| -> String { \"\".to_string() };".parse().unwrap();
    println!("generator_function {}", generator_function);
    let generator_function: syn::Stmt = syn::parse(generator_function).expect("Failed to parse as expr");


    if let syn::Stmt::Local(ref s) = generator_function {
        if let syn::Local { 
            attrs, 
            let_token, 
            pat, 
            init: Some(
                syn::LocalInit {
                    eq_token, 
                    expr, 
                    diverge
                }
            ), 
            semi_token 
        } = s {
            if let syn::Expr::Assign (
                syn::ExprAssign {
                ref attrs,
                ref left,
                eq_token,
                mut ref right,
            }) = **expr {
                right = &Box::new(ast.clone());
            }
        } else {
            panic!("---");
        }
    }
    let tokens = generator_function.to_token_stream().into();
    println!("TOKENS: {}", tokens);
    tokens
}