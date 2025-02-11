#![allow(unused_imports, unused_mut, unused_variables)]

use crate::*;
use syn::punctuated::Punctuated;
use syn::punctuated::Pair;
use syn::Token;

pub fn impl_range_enum(mut ast: syn::DeriveInput) -> TokenStream {
    let mut name = ast.ident.to_string();
    if !name.ends_with("Range") {
        panic!("EnumRange must create new enum from enum whose name ends with Range")
    }
    let _ = name.split_off(name.len() - 5);
    let attrs = ast.attrs.clone();
    for attribute in attrs {
        if let syn::Meta::List(syn::MetaList{ path, delimiter, tokens }) = attribute.meta {
            let path_segments = path.segments.iter()
                .map(|path_segment| path_segment.ident.to_string())
                .collect::<Vec<_>>();
            let p = path_segments.last().unwrap();
            if p == "variant_range" {
                let tokens =  tokens.to_string();
                let s = tokens
                    .split(',')
                    .into_iter().take(2).collect::<Vec<_>>();
                let (start, mut end) = (s[0], s[1]);
                let end = unsafe {
                    let end = end.as_bytes().to_vec().split_off(1);
                    String::from_utf8_unchecked(end)
                };
                let start = start.parse::<usize>().expect(
                    &format!("variant_range: {start} cannot be interpreted as usize").to_string());
                let end = end.parse::<usize>().expect(
                    &format!("variant_range: {end} cannot be interpreted as usize").to_string());
                println!("variant_range: {:?}", (start, end));
            }
        }
    }
    let name = syn::Ident::new(&name, proc_macro2::Span::call_site());
    let mut variant_names: Vec::<syn::Ident>  = vec!();
    let mut p = syn::punctuated::Punctuated::<syn::Variant, Token![,]>::new();
    p.push(syn::Variant {
        attrs: vec!(),
        ident: syn::Ident::new("One", proc_macro2::Span::call_site()),
        fields: syn::Fields::Unit,
        discriminant: None, 
    });
    if let syn::Data::Enum(ref mut data) = &mut ast.data {
        if !data.variants.is_empty(){
            panic!("EnumRange macro can only be used with empty enums");
      
        }
   	    data.variants.extend(p);
        let variants = data.variants.clone().into_iter();
        let quote = quote! {
            enum #name {
                #(#variants,)*
            }
        };
        println!("============{}", quote);
        quote
    } else {
        panic!("EnumRange cannot be derived on non-enum"); 
    }.into()
}