#![allow(dead_code)]
#![allow(unused_imports, unused_mut, unused_variables)]
use syn::Token;
use syn::ItemEnum;
use syn::token;
use syn::token::Comma;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::{Attribute, Ident, Generics, Lit};
use syn::Visibility;
use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::{format_ident};
use quote::TokenStreamExt;
use syn::punctuated::Punctuated;

use proc_macro::TokenTree;

mod range_enum;
use range_enum::*;
use syn::{parse_macro_input, Data, DeriveInput, Variant};


trait Idable {
    const MAX: usize = 0;
    fn idx(&self) -> usize;
}



#[derive(syn_derive::Parse, syn_derive::ToTokens)]
struct GeneratedEnum {
    #[parse(Attribute::parse_outer)]
    #[to_tokens(|tokens, val| tokens.append_all(val))]
    attrs: Vec<Attribute>,

    vis: Visibility,

    enum_token: Token![enum],

    ident: Ident,

    generics: Generics,

    #[syn(braced)]
    brace_token: token::Brace,

    #[syn(in = brace_token)]
    #[parse(Variant::parse, boxed)]
    leading: Box<Variant>,

    #[syn(in = brace_token)]
    template_sep: Token![:],

    #[syn(in = brace_token)]
    template_prefix: Option<Ident>,

    #[syn(in = brace_token)]
    id_place: Token![_],

    #[syn(in = brace_token)]
    template_suffix: Option<Ident>,

    #[syn(in = brace_token)]
    comma: Token![,],

    #[syn(in = brace_token)]
    dots: Token![...],

    #[syn(in = brace_token)]
    #[parse(Variant::parse, boxed)]
    end: Box<Variant>
}


#[proc_macro]
pub fn generate_enum(tokens: TokenStream) -> TokenStream {
    let generated_enum: GeneratedEnum = syn::parse(tokens).expect("Could not parse as GeneratedEnum");
    //let ast: syn::FieldsNamed = syn::parse(tokens).unwrap();
    // let mut generator_function: TokenStream = "pub enum FunctionKey".parse().unwrap();
    let leading_variant = *generated_enum.leading.clone();
    let end_variant = *generated_enum.end.clone();
    if let Some(prefix) = generated_enum.template_prefix.clone() {
        assert!(leading_variant.ident.to_string().starts_with(&prefix.to_string()));
        assert!(end_variant.ident.to_string().starts_with(&prefix.to_string()));
    }

    if let Some(suffix) = generated_enum.template_suffix.clone() {
        assert!(leading_variant.ident.to_string().ends_with(&suffix.to_string()));
        assert!(end_variant.ident.to_string().ends_with(&suffix.to_string()));
    }

    let definition = ItemEnum {
        attrs: generated_enum.attrs,
        vis: generated_enum.vis,
        enum_token: generated_enum.enum_token,
        ident: generated_enum.ident,
        generics: generated_enum.generics,
        brace_token: generated_enum.brace_token,
        variants: make_enum_variants(
            (generated_enum.template_prefix,
            generated_enum.template_suffix),
            leading_variant,
            end_variant)
    };

    println!("{}", quote! { #definition });
    quote! { #definition }.into()
}

fn make_enum_variants(template: (Option<Ident>, Option<Ident>), leading: Variant, end: Variant) -> Punctuated<Variant, Comma> {
    let mut variants = Punctuated::<Variant, Token![,]>::new();
    variants.push(leading.clone());

    let leading_variant_name = leading.ident.to_string();
    let leading_variant_name = leading_variant_name.as_bytes();

    let prefix = if let Some(prefix) = template.0 {
        prefix.to_string()
    } else {
        "".to_string()
    };

    let suffix = if let Some(suffix) = template.1 {
        suffix.to_string()
    } else {
        "".to_string()
    };


    let starting_char = leading_variant_name[prefix.len()..prefix.len()+1][0];
    let start = unsafe { String::from_utf8_unchecked(
        leading_variant_name[prefix.len()..leading_variant_name.len() - suffix.len()].to_vec())
    };
    let name_generator: Box<dyn Fn(usize) -> String> = if let Ok(starting_number) = start.parse::<usize>() {
        let suffix = suffix.clone();
        let number_sequence_gen = move |i: usize| -> String { format!("{}{}{}", &prefix, &(i+starting_number).to_string(), &suffix) };
        Box::new(number_sequence_gen)
    } else {
        if start.len() != 1 {
            panic!("could not identify templating starting index/ starting char.");
        }

        let suffix = suffix.clone();
        let letter_sequence_gen = move |i: usize|  format!("{}{}{}",
            &prefix, 
            &((starting_char + i as u8) as char).to_string(), 
            &suffix);
        Box::new(letter_sequence_gen)
    };

    let mut i = 1;
    loop {
        let mut variant = leading.clone();
        let variant_name = &name_generator(i);
        //println!("{} : {}", variant_name, end.ident.to_string() );
        let ident = Ident::new(&variant_name, proc_macro2::Span::call_site());
        variant.ident = ident;
        variants.push(variant);
        if variants.len() > 500 {
            panic!("GenerateEnum Attempted to generate enum with more than 500 variants");
        } else if *variant_name == end.ident.to_string() {
            break;
        }
        i += 1;
    }

    variants
}



#[proc_macro_derive(RangeEnum, attributes(variant_range, generator_func))]
pub fn derive_range_enum(input: TokenStream) -> TokenStream {
    //let mut ast = syn::parse(input).unwrap();
    let mut ast: DeriveInput = parse_macro_input!(input);
    // Build the trait implementation
    impl_range_enum(ast)
}


#[proc_macro_derive(Idable)]
pub fn derive_idable(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_idable(&ast)
}


fn impl_idable(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let mut variant_names: Vec::<syn::Ident>  = vec!();
    let mut variant_names_field : Vec::<syn::Ident>  = vec!();
    let mut variant_names_named_field : Vec::<syn::Ident>  = vec!();


    let variants = if let  syn::Data::Enum(data) = &ast.data {
        data.variants.iter()
    } else {
        panic!("Idable cannot be derived on non-enum");
    };

    let variant_count = variants.len();
    for variant in variants {
        match &variant.fields {
            syn::Fields::Named(f) => {
                for p in f.named.clone().into_pairs() {
                    let f: syn::Field = p.value().clone();
                    if matches!(f.ty, syn::Type::Verbatim(_)) {
                        println!("{}", f.ty.to_token_stream());
                    } if let syn::Type::Path(p) = f.ty {
                        println!("{:?}", p.path.get_ident().unwrap());
                    } 
                }
                variant_names_named_field.push(variant.ident.clone())
            },
            syn::Fields::Unnamed(_) => variant_names_field.push(variant.ident.clone()),
            syn::Fields::Unit => variant_names.push(variant.ident.clone())
        }
    }


    let mut indexes=vec!();
    let mut indexes_field=vec!();
    let mut indexes_named_field=vec!();
    for i in 0..variant_names.len() {
        indexes.push(i);
    }
    let end = variant_names.len() + variant_names_field.len();
    for i in variant_names.len()..end {
        indexes_field.push(i);
    }
    for i in end..(end + variant_names_named_field.len()) {
        indexes_named_field.push(i);
    }

    let arm =  format_match_arm(syn::Ident::new("vafgg", proc_macro2::Span::call_site()));
    let gen = quote! {

        impl Idable for #name {
            const MAX: usize = #variant_count;
            fn idx(&self) -> usize {
                match self {
                    #(Self::#variant_names => #indexes,)* 
                    #(Self::#variant_names_field(..) => #indexes_field,)* 
                    #(Self::#variant_names_named_field{..} => #indexes_named_field,)* 
                }
            }
        }
    };
    print!("{}", gen);
    gen.into()
}

fn format_match_arm(ident: syn::Ident) -> syn::Arm {
    let pat = syn::PatIdent {
        attrs: vec!(),
        by_ref: None,
        mutability: None,
        ident: ident,
        subpat: None,
    };

    syn::Arm {
        attrs: vec!(),
        pat: syn::Pat::Ident(pat),
        guard: None,
        fat_arrow_token: <syn::Token![=>]>::default(),
        body: Box::new(format_expression()),
        comma: Some(<syn::Token![,]>::default()),
    }
}

fn format_expression() -> syn::Expr {

    let mut statements = vec!();
    let statement = make_statements();
    statements.push(statement);
    let block = syn::Expr::Block(
        syn::ExprBlock {
            attrs: vec!(),
            label: None,
            block: syn::Block {
                stmts: statements,
                brace_token: Default::default()
            },
        }
    );
    block
}

fn make_statements() -> syn::Stmt {
    syn::Stmt::Local(
        syn::Local {
            attrs: vec!(),
            let_token: <syn::Token![let]>::default(),
            pat: 
                syn::Pat::Ident( 
                    syn::PatIdent{
                    attrs: vec!(), 
                    by_ref: None, 
                    mutability:  None, 
                    ident: format_ident!("v1"), 
                    subpat: None
                }),
            init: Some(
                syn::LocalInit {
                    eq_token: <syn::Token![=]>::default(),
                    expr: Box::new(
                        syn::Expr::Binary(
                            syn::ExprBinary {
                                attrs: vec!(),
                                left: Box::new(syn::Expr::Lit(
                                    syn::ExprLit {
                                         attrs: vec!(), 
                                         lit: syn::Lit::Int(syn::LitInt::new("2", proc_macro2::Span::call_site() )),
                                    })),
                                right:Box::new(syn::Expr::Lit(
                                    syn::ExprLit {
                                         attrs: vec!(), 
                                         lit: syn::Lit::Int(syn::LitInt::new("2", proc_macro2::Span::call_site() )),
                                    })),
                                op: syn::BinOp::Add(<syn::Token![+]>::default())
                        })),
                    diverge: None,
            }),
            semi_token: <syn::Token![;]>::default(),
        }
    )
}
//Self::ExampleField3(v1, _, v2, _, _, v3) => {
//#totals  + 
//v3 + v2 * v3.max() + v1 * (v2.max() * v3.max() + v3.max())
//                    } 

